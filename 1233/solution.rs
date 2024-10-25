impl Solution {
    pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
        // Sort the folders by name
        let mut folder = folder;
        folder.sort_unstable();

        let mut result = vec![folder[0].clone()];
        let mut last = &folder[0];

        // Iterate through the sorted folders
        for i in 1..folder.len() {
            // If the current folder does not start with the last folder (not a sub-folder)
            if !folder[i].starts_with(&format!("{}/", last)) {
                // Add the current folder to the result
                result.push(folder[i].clone());
                last = &folder[i];
            }
        }

        // Return the resultF
        result
    }
}
