package main

import (
	"context"
	"fmt"
	"log"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"time"

	"github.com/go-chi/chi/v5"
	chimiddleware "github.com/go-chi/chi/v5/middleware"
	"github.com/travillian/tusk-horn/internal/config"
	"github.com/travillian/tusk-horn/internal/pkg/database"
	"github.com/travillian/tusk-horn/internal/pkg/firebase"
	"github.com/travillian/tusk-horn/internal/pkg/logger"
	"github.com/travillian/tusk-horn/internal/pkg/telemetry"
	"github.com/travillian/tusk-horn/internal/server/middleware"
)

func main() {
	// 0. Load Config (First to get log level)
	cfg, err := config.Load()
	if err != nil {
		log.Fatalf("Failed to load config: %v", err)
	}

	// 1. Init Logger
	logger.Init(cfg.App.LogLevel, cfg.App.Env)
	logger.Log.Info("Tusk & Horn Server Starting...", "env", cfg.App.Env, "port", cfg.App.Port)

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	// 2. Setup Telemetry
	shutdownTracer, err := telemetry.InitTracer(ctx, cfg.OTEL)
	if err != nil {
		logger.Log.Error("Failed to init tracer", "error", err)
	}
	defer func() {
		if err := shutdownTracer(context.Background()); err != nil {
			logger.Log.Error("Error shutting down tracer", "error", err)
		}
	}()

	// 3. Database Connections
	// Postgres
	pg, err := database.NewPostgres(cfg.Postgres)
	if err != nil {
		logger.Log.Error("Failed to connect to Postgres", "error", err)
		os.Exit(1)
	}
	defer pg.Close()

	// Redis
	rdb, err := database.NewRedis(cfg.Redis)
	if err != nil {
		logger.Log.Error("Failed to connect to Redis", "error", err)
		os.Exit(1)
	}
	defer rdb.Close()

	// 4. Initialize Firebase
	fbClient, err := firebase.NewClient(cfg.Firebase.CredentialsPath)
	if err != nil {
		logger.Log.Warn("Failed to init Firebase. Auth will fail.", "error", err)
	} else {
		_ = fbClient
	}

	// 5. Setup Router
	r := chi.NewRouter()
	r.Use(middleware.Cors(cfg.App.AllowOrigins))
	r.Use(middleware.Logger) // Custom structured logger
	r.Use(chimiddleware.Recoverer)
	r.Use(chimiddleware.RequestID)
	r.Use(chimiddleware.RealIP)

	r.Get("/health", func(w http.ResponseWriter, r *http.Request) {
		if err := pg.Pool.Ping(r.Context()); err != nil {
			http.Error(w, "Database unavailable", http.StatusServiceUnavailable)
			return
		}
		w.WriteHeader(http.StatusOK)
		w.Write([]byte("OK"))
	})

	// 6. Start Server
	server := &http.Server{
		Addr:    fmt.Sprintf(":%s", cfg.App.Port),
		Handler: r,
	}

	go func() {
		log.Printf("Server listening on %s", server.Addr)
		if err := server.ListenAndServe(); err != nil && err != http.ErrServerClosed {
			log.Fatalf("Server failed: %v", err)
		}
	}()

	// Graceful Shutdown
	stop := make(chan os.Signal, 1)
	signal.Notify(stop, os.Interrupt, syscall.SIGTERM)
	<-stop

	log.Println("Shutting down server...")
	shutdownCtx, shutdownCancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer shutdownCancel()

	if err := server.Shutdown(shutdownCtx); err != nil {
		log.Fatalf("Server forced to shutdown: %v", err)
	}

	log.Println("Server exited properly")
}
