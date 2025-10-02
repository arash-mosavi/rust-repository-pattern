// Module declarations - using new Go-style structure
mod pkg;
mod adapters;
mod internal;

use std::env;
use std::sync::Arc;

use pkg::RepositoryError;
use internal::modules::users::{
    delivery::http::dto::{CreateUserDto, UpdateUserDto},
    repositories::InMemoryUserRepository,
    service::UserService,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file (if exists)
    dotenvy::dotenv().ok();

    // Check if we should use PostgreSQL or in-memory
    let use_postgres = env::var("USE_POSTGRES").unwrap_or_else(|_| "false".to_string()) == "true";

    if use_postgres {
        println!("=== Repository Pattern Example (PostgreSQL) ===\n");
        run_with_postgres().await
    } else {
        println!("=== Repository Pattern Example (In-Memory) ===\n");
        run_with_in_memory().await
    }
}

async fn run_with_in_memory() -> Result<(), Box<dyn std::error::Error>> {
    // Create the user service with in-memory repository
    let repository = Arc::new(InMemoryUserRepository::new());
    let service = Arc::new(UserService::new(repository));

    // Run examples
    run_examples(service).await
}

async fn run_with_postgres() -> Result<(), Box<dyn std::error::Error>> {
    println!("Connecting to database...");
    
    // Note: PostgreSQL repository not yet implemented in new structure
    println!("PostgreSQL support not yet migrated to new module structure.");
    println!("Using in-memory repository instead.\n");
    
    run_with_in_memory().await
}

async fn run_examples<R: internal::modules::users::repositories::UserRepository + Send + Sync>(
    service: Arc<UserService<R>>,
) -> Result<(), Box<dyn std::error::Error>> {

    // Example 1: Create users
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

    // Example 2: Try to create a user with duplicate username
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

    // Example 3: Get user by ID
    println!("3. Getting user by ID...");
    let fetched_user = service.get_user(user1.id).await?;
    println!("   Found: {} - {}\n", fetched_user.full_name, fetched_user.email);

    // Example 4: Find user by username
    println!("4. Finding user by username...");
    if let Some(user) = service.find_by_username("jane_smith").await? {
        println!("   Found: {} ({})\n", user.full_name, user.email);
    }

    // Example 5: Get all users
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

    // Example 6: Update a user
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

    // Example 7: Find users by age range
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

    // Example 8: Get statistics
    println!("8. Getting user statistics...");
    let stats = service.get_statistics().await?;
    println!("   Total Users: {}", stats.total_users);
    println!("   Users with Age: {}", stats.users_with_age);
    if let Some(avg) = stats.average_age {
        println!("   Average Age: {:.1}\n", avg);
    } else {
        println!("   Average Age: N/A\n");
    }

    // Example 9: Delete a user
    println!("9. Deleting user...");
    let deleted = service.delete_user(user3.id).await?;
    if deleted {
        println!("   User {} deleted successfully", user3.username);
    }

    let all_remaining = service.get_all_users().await?;
    println!("   Remaining users: {}\n", all_remaining.len());

    // Example 10: Try to get deleted user
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
