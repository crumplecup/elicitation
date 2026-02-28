# Planning Documents

This file tracks all planning documents for the elicitation project.

## Archive Notice

**All planning documents archived as of v0.7.0** (commit `98ad6f91b10ee273027ea07d5069da4d90a37e97`)

All previously tracked planning documents have been deleted from the working tree as they are now out of date. The complete history of all planning documents is preserved in git history. To view any archived document:

```bash
git show 98ad6f91b10ee273027ea07d5069da4d90a37e97:<filename>
```

Example:
```bash
git show 98ad6f91b10ee273027ea07d5069da4d90a37e97:UTF8_VERIFICATION_STRATEGY.md
git show 98ad6f91b10ee273027ea07d5069da4d90a37e97:KANI_VERIFICATION_PATTERNS.md
git show 98ad6f91b10ee273027ea07d5069da4d90a37e97:elicitation_vision.md
```

---

## Current Active Plans

### Method Reflection (v0.9.0+)

**Document:** [METHOD_REFLECTION_PLAN.md](METHOD_REFLECTION_PLAN.md)

**Status:** Planning

**Description:** Automatic MCP tool generation for third-party crate methods through newtype-based method reflection. Enables one-line integration of any Rust library as verified AI tools.

**Key Features:**
- `elicit_newtype!` macro for transparent wrapper generation
- `#[reflect_methods]` attribute for automatic method discovery
- Smart &T → T conversion for borrowed parameters
- JsonSchema-bounded generic support
- Seamless integration with existing `#[derive(Elicit)]`

**Timeline:** 6-week phased implementation (5 milestones)

New plans can be added here as needed for future development.
