use std::collections::HashMap;

// --- Data Structures (Blueprints for our data) ---
struct UserProfile {
    authority: String,
    username: String,
    note_count: u64,
}

struct Note {
    authority: String,
    id: u64,
    title: String,
    content: String,
}

// --- Helper Functions to Generate Storage Keys ---
// This function simulates calculating the predictable address for a user's profile.
fn get_user_profile_key(user_authority: &String) -> String {
    format!("{}_user_profile", user_authority)
}

// This function simulates calculating the predictable address for a specific note.
fn get_note_key(user_authority: &String, note_id: u64) -> String {
    format!("{}_note_{}", user_authority, note_id)
}

// --- The CRUD Functions (Our "Instructions") ---

fn create_user(
    signer: String,
    username: String,
    user_db: &mut HashMap<String, UserProfile>,
) {
    println!("--- ACTION: CREATE USER ---");

    let user_profile_key = get_user_profile_key(&signer);

    if user_db.contains_key(&user_profile_key) {
        println!("Error: User profile already exists.\n");
        return;
    }

    let new_profile = UserProfile {
        authority: signer.clone(),
        username: username,
        note_count: 0,
    };

    user_db.insert(user_profile_key, new_profile);
    println!("Success: User profile created for '{}'.\n", signer);
}

fn create_note(
    signer: String,
    title: String,
    content: String,
    user_db: &mut HashMap<String, UserProfile>,
    note_db: &mut HashMap<String, Note>,
) {
    println!("--- ACTION: CREATE NOTE ---");
    let user_profile_key = get_user_profile_key(&signer);

    if let Some(profile) = user_db.get_mut(&user_profile_key) {
        let new_note_id = profile.note_count + 1;
        
        let new_note = Note {
            authority: signer.clone(),
            id: new_note_id,
            title: title,
            content: content,
        };

        let note_key = get_note_key(&signer, new_note_id);
        note_db.insert(note_key, new_note);

        profile.note_count += 1;
        println!("Success: Note #{} created.\n", new_note_id);
    } else {
        println!("Error: User profile not found.\n");
    }
}

fn update_note(
    signer: String,
    note_id: u64,
    new_content: String,
    note_db: &mut HashMap<String, Note>,
) {
    println!("--- ACTION: UPDATE NOTE #{} ---", note_id);
    let note_key = get_note_key(&signer, note_id);

    if let Some(note) = note_db.get_mut(&note_key) {
        if note.authority == signer {
            note.content = new_content;
            println!("Success: Note updated.\n");
        } else {
            println!("Error: You are not the owner of this note.\n");
        }
    } else {
        println!("Error: Note not found.\n");
    }
}

fn delete_note(signer: String, note_id: u64, note_db: &mut HashMap<String, Note>) {
    println!("--- ACTION: DELETE NOTE #{} ---", note_id);
    let note_key = get_note_key(&signer, note_id);

    if let Some(note) = note_db.get(&note_key) {
        if note.authority == signer {
            note_db.remove(&note_key);
            println!("Success: Note deleted.\n");
        } else {
            println!("Error: You are not the owner of this note.\n");
        }
    } else {
        println!("Error: Note not found.\n");
    }
}

fn print_user_notes(
    user_authority: &String,
    user_db: &HashMap<String, UserProfile>,
    note_db: &HashMap<String, Note>,
) {
    println!("--- ACTION: READ ALL NOTES for '{}' ---", user_authority);
    let user_profile_key = get_user_profile_key(user_authority);
    
    if let Some(profile) = user_db.get(&user_profile_key) {
        println!("(User '{}' has created a total of {} notes)", profile.username, profile.note_count);
        
        let mut notes_found = 0;
        for i in 1..=profile.note_count {
            let note_key = get_note_key(user_authority, i);
            if let Some(note) = note_db.get(&note_key) {
                notes_found += 1;
                println!(
                    "  -> Note ID: {}, Title: '{}', Content: '{}'",
                    note.id, note.title, note.content
                );
            }
        }
        if notes_found == 0 {
            println!("  (User has no active notes to display)");
        }
        println!("");
    } else {
        println!("Error: Could not find user profile.\n");
    }
}

// =======================================================
// The Main Function Where We Run Our Simulation
// =======================================================
fn main() {
    let mut user_profiles: HashMap<String, UserProfile> = HashMap::new();
    let mut notes: HashMap<String, Note> = HashMap::new();

    let alice = "alice_wallet_address".to_string();
    let bob = "bob_wallet_address".to_string();

    // --- Alice's Actions ---
    println!("--- SIMULATION START: ALICE'S ACTIONS ---\n");
    create_user(alice.clone(), "Alice".to_string(), &mut user_profiles);
    create_note(alice.clone(), "Groceries".to_string(), "Milk and bread".to_string(), &mut user_profiles, &mut notes);
    create_note(alice.clone(), "To-Do".to_string(), "Learn Rust".to_string(), &mut user_profiles, &mut notes);
    print_user_notes(&alice, &user_profiles, &notes);
    update_note(alice.clone(), 1, "Milk, bread, and eggs".to_string(), &mut notes);
    delete_note(alice.clone(), 2, &mut notes);
    print_user_notes(&alice, &user_profiles, &notes);

    // --- Bob's Actions ---
    println!("--- SIMULATION START: BOB'S ACTIONS ---\n");
    create_user(bob.clone(), "Bob".to_string(), &mut user_profiles);
    create_note(bob.clone(), "Work Reminders".to_string(), "Finish report".to_string(), &mut user_profiles, &mut notes);
    print_user_notes(&bob, &user_profiles, &notes);

    // --- Security Test ---
    println!("--- SIMULATION START: SECURITY TEST ---\n");
    println!("Bob tries to delete Alice's note #1...");
    delete_note(bob.clone(), 1, &mut notes);

    // --- Final State ---
    println!("--- FINAL STATE OF THE DATABASES ---\n");
    println!("User Profiles DB has {} entries.", user_profiles.len());
    println!("Notes DB has {} entries.", notes.len());
}