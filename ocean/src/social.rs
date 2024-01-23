use std::collections::HashMap;

#[derive(Debug)]
pub struct SocialNetwork {
    friendships: HashMap<String, Vec<String>>
}

impl SocialNetwork {
    pub fn new() -> SocialNetwork {
        Self {
            friendships: HashMap::new(),
        }
    }

    /*  Attempts to add a crab as a friend.
    
    # Arguments
    * `crab_id` - The identifier of the crab adding a friend.
    * `friend_id` - The identifier of the crab being added as a friend.
    
    # Returns
    * Result enum
    */
    pub fn add_friend(&mut self, crab_id: String, friend_id: String) -> Result<(), String> {
        if crab_id == friend_id {
            // Prevent a crab from being friends with itself
            return Err("A crab cannot befriend itself".to_string());
        }

        match self.friendships.get_mut(&crab_id) {
            Some(friends) => {
                if friends.contains(&friend_id) {
                    // The friend is already in the list
                    Err("Already friends".to_string())
                } else {
                    // Add the new friend
                    friends.push(friend_id);
                    Ok(())
                }
            }
            None => {
                // This crab doesn't have a friends list yet, create one
                self.friendships.insert(crab_id, vec![friend_id]);
                Ok(())
            }
        }
    }

    /*
    Attempts to remove a crab as a friend.

    # Arguments
    * `crab_id` - The identifier of the crab removing a friend.
    * `friend_id` - The identifier of the friend to be removed.

    # Returns
    * Result enum
    */
    pub fn remove_friend(&mut self, crab_id: String, friend_id: String) -> Result<(), String> {
        match self.friendships.get_mut(&crab_id) {
            Some(friends) => {
                if friends.contains(&friend_id) {
                    friends.retain(|f| f != &friend_id);
                    Ok(())  // Friend was found and removed
                } else {
                    Err("Friend not found in list".to_string())  // Friend was not in the list
                }
            },
            None => Err("Crab ID does not exist or has no friends".to_string())  // Crab ID does not exist or has no friends
        }
    }

    /*
    Returns True if crab2 is a friend of crab1, else False.
    */
    pub fn is_friend(&self, crab1_id: String, crab2_id: String) -> bool {
        match self.friendships.get(&crab1_id) {
            Some(friends) => {
                if friends.contains(&crab2_id) {
                    return true
                } else {
                    return false
                }
            },
            None => false 
        }
    }

}