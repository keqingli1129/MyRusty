// The `video` module: the Video type from docs/Ownership.txt ("References to private fields"),
// plus the practice exercise, which adds like and dislike counts.

// A Video holds a String, and String is move-only because it owns a heap allocation.
// So Video can't be Copy either. Adding #[derive(Clone, Copy)] here fails with:
//     error[E0204]: the trait `Copy` cannot be implemented for this type
pub struct Video {
    title: String, // private (no `pub`): only code in this module can touch it
    likes: u32, // exercise: number of likes
    dislikes: u32, // exercise: number of dislikes
}

impl Video {
    // Factory function. The title field is private, so this is the only way to make a Video.
    pub fn new(title: String) -> Video {
        // `title` on its own is shorthand for `title: title`, because the parameter has the
        // same name as the field. The counts start at zero.
        Video {
            title,
            likes: 0,
            dislikes: 0,
        }
    }

    // Read-only access to the private title. `&self` works for callers who own the Video,
    // have exclusive access to it, or only have a shared reference: the most flexible choice.
    //
    // The versions tried in the video, in order:
    //   fn title(&self) -> String { self.title }
    //       error[E0507]: cannot move out of `self.title` which is behind a shared reference
    //       (same error with &mut self: the Video must still be valid after the call)
    //   fn title(&self) -> String { self.title.clone() }
    //       compiles, but makes a new heap allocation and copies every byte
    //   fn title(&self) -> &String { &self.title }
    //       no copy, but tells callers we store a String, so changing that would break them
    //
    // Final version: return a string slice. It points straight at the UTF-8 bytes, and any
    // string type can provide one, so callers don't depend on how we store the title.
    pub fn title(&self) -> &str {
        &self.title // a &String automatically coerces to &str
    }

    // ---- Practice exercise ----
    // Methods that change fields take `&mut self`; methods that only read take `&self`.

    // Count one like.
    pub fn add_like(&mut self) {
        self.likes += 1; // increment the like count
    }

    // Count one dislike.
    pub fn add_dislike(&mut self) {
        self.dislikes += 1; // increment the dislike count
    }

    // Return both counts together as a tuple: (likes, dislikes).
    pub fn likes_dislikes(&self) -> (u32, u32) {
        (self.likes, self.dislikes) // tuple expression
    }

    // Fraction of all votes that are likes, e.g. 3 likes and 1 dislike gives 0.75.
    pub fn like_ratio(&self) -> f64 {
        let total = self.likes + self.dislikes; // all votes
        // Cast both to f64 so this is floating-point division, not integer division.
        // With no votes this is 0.0 / 0.0, which gives NaN ("not a number"):
        // floats don't panic on division by zero, unlike integers.
        self.likes as f64 / total as f64
    }
}

// The exercise's unit test: three likes and one dislike.
#[test]
fn test_likes_dislikes() {
    let mut v = Video::new(String::from("Structs")); // `mut` because add_like needs &mut self
    v.add_like(); // 1 like
    v.add_like(); // 2 likes
    v.add_like(); // 3 likes
    v.add_dislike(); // 1 dislike
    assert_eq!(v.likes_dislikes(), (3, 1)); // compare whole tuples
    assert_eq!(v.like_ratio(), 0.75); // 3 / (3 + 1); exact, since 0.75 is representable
    assert_eq!(v.title(), "Structs"); // the title is still readable
}
