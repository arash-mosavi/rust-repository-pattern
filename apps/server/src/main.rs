use std::sync::Arc;
use std::env;


use pkg::{init_logging, RepositoryError};
use core_config::AppConfig;
use core_db::DatabaseFactory;
use users_module::{
    delivery::http::{create_user_router, dto::{CreateUserDto, UpdateUserDto}},
    repositories::InMemoryUserRepository,
    service::UserService,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    dotenvy::dotenv().ok();


    init_logging();


    let config = AppConfig::from_env()?;


    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        match args[1].as_str() {
            "serve" | "server" | "http" => {
                run_http_server(config).await
            }
            "cli" | "demo" => {
                run_cli_demo(config).await
            }
            "migrate" => {
                run_migrations(config).await
            }
            "migrate:status" | "migration:status" => {
                show_migration_status(config).await
            }
            "migrate:list" | "migration:list" => {
                list_migrations().await
            }
            _ => {
                println!("Unknown command: {}", args[1]);
                print_usage();
                Ok(())
            }
        }
    } else {

        run_http_server(config).await
    }
}

fn print_usage() {
    println!("Usage: server [COMMAND]");
    println!();
    println!("Commands:");
    println!("  serve, server, http      - Start HTTP API server (default)");
    println!("  cli, demo                - Run CLI demo");
    println!("  migrate                  - Run database migrations");
    println!("  migrate:status           - Show migration status");
    println!("  migrate:list             - List all available migrations");
    println!();
    println!("Environment Variables:");
    println!("  DATABASE_URL         - PostgreSQL connection string");
    println!("  SERVER_HOST          - Server host (default: 0.0.0.0)");
    println!("  SERVER_PORT          - Server port (default: 3000)");
    println!("  USE_POSTGRES         - Use PostgreSQL instead of in-memory (true/false)");
}

async fn run_http_server(config: AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("🚀 Starting User API Server...");


    let repository = Arc::new(InMemoryUserRepository::new());
    let service = Arc::new(UserService::new(repository));


    let app = create_user_router(service);


    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("✅ Server running on http://{}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}

async fn run_cli_demo(_config: AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Repository Pattern CLI Demo ===\n");


    let repository = Arc::new(InMemoryUserRepository::new());
    let service = Arc::new(UserService::new(repository));


    run_examples(service).await
}

async fn run_migrations(config: AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    use core_db::MigrationRunner;

    println!("🚀 Starting code-first database migrations...\n");

    let pool = DatabaseFactory::create_postgres_pool(&config.database).await?;
    


    let all_migrations: Vec<_> = vec![
        users_module::USER_MIGRATIONS,



    ]
    .into_iter()
    .flatten()
    .copied()
    .collect();
    

    let runner = MigrationRunner::new(pool);
    runner.run_migrations(&all_migrations).await?;

    println!("\n✅ Migration process completed successfully!");

    Ok(())
}

async fn run_examples<R: users_module::repositories::UserRepository + Send + Sync>(
    service: Arc<UserService<R>>,
) -> Result<(), Box<dyn std::error::Error>> {

    println!("1. Creating users...");
    let user1_dto = CreateUserDto {
        username: "john_doe".to_string(),
        email: "john@example.com".to_string(),
        full_name: "John Doe".to_string(),
        age: Some(30),
    };

    let user1 = service.create_user(user1_dto).await?;
    println!("   Created user: {} (ID: {})", user1.username, user1.id);

    let user2_dto = CreateUserDto {
        username: "jane_smith".to_string(),
        email: "jane@example.com".to_string(),
        full_name: "Jane Smith".to_string(),
        age: Some(25),
    };

    let user2 = service.create_user(user2_dto).await?;
    println!("   Created user: {} (ID: {})", user2.username, user2.id);

    let user3_dto = CreateUserDto {
        username: "bob_wilson".to_string(),
        email: "bob@example.com".to_string(),
        full_name: "Bob Wilson".to_string(),
        age: Some(35),
    };

    let user3 = service.create_user(user3_dto).await?;
    println!("   Created user: {} (ID: {})\n", user3.username, user3.id);


    println!("2. Trying to create user with duplicate username...");
    let duplicate_dto = CreateUserDto {
        username: "john_doe".to_string(),
        email: "different@example.com".to_string(),
        full_name: "Different User".to_string(),
        age: Some(40),
    };

    match service.create_user(duplicate_dto).await {
        Ok(_) => println!("   Unexpected success!"),
        Err(RepositoryError::ValidationError(msg)) => {
            println!("   Expected error: {}\n", msg)
        }
        Err(e) => println!("   Unexpected error: {}\n", e),
    }


    println!("3. Getting user by ID...");
    let fetched_user = service.get_user(user1.id).await?;
    println!("   Found: {} - {}\n", fetched_user.full_name, fetched_user.email);


    println!("4. Finding user by username...");
    if let Some(user) = service.find_by_username("jane_smith").await? {
        println!("   Found: {} ({})\n", user.full_name, user.email);
    }


    println!("5. Getting all users...");
    let all_users = service.get_all_users().await?;
    println!("   Total users: {}", all_users.len());
    for user in &all_users {
        println!(
            "   - {} (@{}) - Age: {:?}",
            user.full_name, user.username, user.age
        );
    }
    println!();


    println!("6. Updating user...");
    let update_dto = UpdateUserDto {
        username: None,
        email: Some("john.doe.updated@example.com".to_string()),
        full_name: Some("John Doe Updated".to_string()),
        age: Some(31),
    };

    let updated_user = service.update_user(user1.id, update_dto).await?;
    println!(
        "   Updated: {} - {}\n",
        updated_user.full_name, updated_user.email
    );


    println!("7. Finding users by age range (25-32)...");
    let users_in_range = service.get_users_by_age_range(25, 32).await?;
    println!("   Found {} users:", users_in_range.len());
    for user in users_in_range {
        println!(
            "   - {} (Age: {})",
            user.full_name,
            user.age.unwrap_or(0)
        );
    }
    println!();


    println!("8. Getting user statistics...");
    let stats = service.get_statistics().await?;
    println!("   Total Users: {}", stats.total_users);
    println!("   Users with Age: {}", stats.users_with_age);
    if let Some(avg) = stats.average_age {
        println!("   Average Age: {:.1}\n", avg);
    } else {
        println!("   Average Age: N/A\n");
    }


    println!("9. Deleting user...");
    let deleted = service.delete_user(user3.id).await?;
    if deleted {
        println!("   User {} deleted successfully", user3.username);
    }

    let all_remaining = service.get_all_users().await?;
    println!("   Remaining users: {}\n", all_remaining.len());


    println!("10. Trying to get deleted user...");
    match service.get_user(user3.id).await {
        Ok(_) => println!("   Unexpected success!"),
        Err(RepositoryError::NotFound(id)) => {
            println!("   Expected error: User with ID {} not found\n", id)
        }
        Err(e) => println!("   Unexpected error: {}\n", e),
    }

    println!("=== Repository Pattern Demo Complete ===");

    Ok(())
}


async fn show_migration_status(config: AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    use core_db::MigrationRunner;

    println!("📊 Migration Status Report\n");
    println!("═══════════════════════════════════════════════════════════════\n");

    let pool = DatabaseFactory::create_postgres_pool(&config.database).await?;
    let runner = MigrationRunner::new(pool);

    match runner.get_status().await {
        Ok(statuses) => {
            if statuses.is_empty() {
                println!("❌ No migrations have been applied yet.");
                println!("\n💡 Run 'cargo run -p server migrate' to apply migrations.");
            } else {
                println!("✅ Found {} applied migration(s):\n", statuses.len());


                let mut by_module: std::collections::HashMap<String, Vec<_>> = std::collections::HashMap::new();
                for status in statuses {
                    by_module.entry(status.module.clone()).or_insert_with(Vec::new).push(status);
                }

                for (module, migrations) in by_module.iter() {
                    println!("📦 Module: {}", module);
                    println!("   ├─ {} migration(s) applied", migrations.len());
                    
                    for migration in migrations {
                        println!("   │");
                        println!("   ├─ Version: {}", migration.version);
                        println!("   │  Name: {}", migration.name);
                        println!("   │  Applied: {}", migration.applied_at);
                        println!("   │  Execution: {}ms", migration.execution_time_ms);
                    }
                    println!();
                }

                println!("═══════════════════════════════════════════════════════════════");
            }
        }
        Err(e) => {
            eprintln!("❌ Error fetching migration status: {}", e);
            return Err(Box::new(e));
        }
    }

    Ok(())
}


async fn list_migrations() -> Result<(), Box<dyn std::error::Error>> {
    println!("📋 Available Migrations\n");
    println!("═══════════════════════════════════════════════════════════════\n");

    let all_migrations: Vec<_> = vec![
        users_module::USER_MIGRATIONS,

    ]
    .into_iter()
    .flatten()
    .collect();

    if all_migrations.is_empty() {
        println!("❌ No migrations found.");
    } else {
        println!("✅ Found {} total migration(s):\n", all_migrations.len());


        let mut by_module: std::collections::HashMap<&str, Vec<_>> = std::collections::HashMap::new();
        for migration in &all_migrations {
            by_module.entry(migration.module).or_insert_with(Vec::new).push(*migration);
        }

        for (module, migrations) in by_module.iter() {
            println!("📦 Module: {}", module);
            println!("   ├─ {} migration(s) defined", migrations.len());
            
            for migration in migrations {
                println!("   │");
                println!("   ├─ Version: {}", migration.version);
                println!("   │  Name: {}", migration.name);
                println!("   │  ID: {}", migration.id());
                println!("   │  Checksum: {}", migration.checksum());
                let sql_preview = migration.sql.lines().next().unwrap_or("").trim();
                println!("   │  SQL Preview: {}...", 
                    if sql_preview.len() > 60 { 
                        &sql_preview[..60] 
                    } else { 
                        sql_preview 
                    }
                );
            }
            println!();
        }

        println!("═══════════════════════════════════════════════════════════════");
        println!("\n💡 Run 'cargo run -p server migrate' to apply these migrations.");
        println!("💡 Run 'cargo run -p server migrate:status' to see which are applied.");
    }

    Ok(())
}
