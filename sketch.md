There's no impl<T: KaniCompose, const N: usize> KaniCompose for [T; N]. Blackjack's phase structs use fixed-size arrays like [Hand;
MAX_PLAYER_HANDS], [u64; MAX_PLAYER_HANDS], and [Outcome; MAX_PLAYER_HANDS] — when #[derive(KaniCompose)] encounters those fields, it falls through to the generic
delegation path <[Hand; MAX_PLAYER_HANDS] as KaniCompose>::kani_depth0(), but there's no such impl, so it fails to compile under Kani.

The fix is a const-generic blanket impl in kani_compose.rs:

 #[cfg(kani)]
 impl<T: KaniCompose, const N: usize> KaniCompose for [T; N] {
     fn kani_depth0() -> Self {
         std::array::from_fn(|_| T::kani_depth0())
     }
     fn kani_depth1() -> Self {
         std::array::from_fn(|_| T::kani_depth1())
     }
     fn kani_depth2() -> Self {
         std::array::from_fn(|_| T::kani_depth2())
     }
 }

std::array::from_fn (stable since 1.63) doesn't require Copy — it calls the closure N times, so T just needs KaniCompose. No changes needed to the derive macros;
the fallthrough already generates the right call site expression.

The NodeJson builder doesn't expose is_selected (it's Option<bool> in accesskit::Node but not surfaced in NodeJson), and the ratatui bridge
only reads it for Role::TreeItem. To carry cursor position properly through the IR — so ratatui, egui, and leptos can all render it — we need two upstream
patches:

 1. elicit_accesskit: Add is_selected: Option<bool> to NodeJson + with_selected(bool) builder method, round-tripping through accesskit::Node::is_selected()
 2. elicit_ratatui: In bridge_paragraph (or a new bridge_cell), check node.is_selected() and apply a highlight style (e.g. reverse video / bold) when Some(true)

Then in to_ak_nodes for the TTT board, the cell at cursor position gets .with_selected(true) — semantically: "this cell is focused/active". Egui and Leptos
bridges would then render it with an outline or CSS class respectively.
