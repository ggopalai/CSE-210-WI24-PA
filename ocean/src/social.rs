#[derive(Debug)]
pub struct SocialNetwork {
    //TO-DO: Add appropriate field
}

impl SocialNetwork {
    pub fn new() -> SocialNetwork {
    }

    /*  Attempts to add a crab as a friend.
    
    # Arguments
    * `crab_id` - The identifier of the crab adding a friend.
    * `friend_id` - The identifier of the crab being added as a friend.
    
    # Returns
    * Result enum
    */
    pub fn add_friend(&mut self, crab_id: String, friend_id: String) -> Result<(), String> {
        unimplemented!();
        // test 
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
        unimplemented!();
        // test 
    }

    /*
    Returns True if crab2 is a friend of crab1, else False.
    */
    pub fn is_friend(&self, crab_id: String, friend_id: String) -> bool {
        unimplemented!();
        // test
    }

}