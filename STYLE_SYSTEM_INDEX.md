# Style System Documentation Index

## 📚 Documents

### 1. **STYLE_SYSTEM_RESEARCH.md** (Executive Summary - 4 min read)
   - Overview of the system architecture
   - 11 critical findings with code snippets
   - Key files to review
   - Essential type signatures
   - Quick conclusion

### 2. **STYLE_SYSTEM_QUICK_REF.md** (Quick Reference - Lookup Guide)
   - One-liner definition
   - 5-minute overview
   - Type signatures
   - File locations (with line numbers)
   - Built-in styles comparison table
   - Macro generation patterns
   - Complete elicitation flow
   - Blackjack game example
   - Design principles
   - TUI integration roadmap

### 3. **STYLE_SYSTEM_DEEP_DIVE.md** (Comprehensive Analysis - 30 min read)
   - 11 detailed sections with full code examples
   - Complete trait hierarchies
   - Macro generation walkthrough
   - Storage design explanation
   - Communication flow architecture
   - Type signature specifications
   - Design principles explained

---

## 🎯 Quick Navigation

**I want to...**

| Goal | Document | Section |
|------|----------|---------|
| Understand what Style is | RESEARCH.md | Intro |
| See an example quickly | QUICK_REF.md | Concrete Example |
| Understand the flow | QUICK_REF.md | FLOW section |
| Learn trait bounds | QUICK_REF.md | TYPE SIGNATURES |
| Find a code location | QUICK_REF.md | FILES & LOCATIONS table |
| Understand macro generation | DEEP_DIVE.md | Section 3 |
| Learn StyleContext | QUICK_REF.md | STYLE CONTEXT |
| See design principles | QUICK_REF.md | DESIGN PRINCIPLES |
| Implement custom style | DEEP_DIVE.md | Section 7 or QUICK_REF.md |

---

## 🔍 Key Concepts at a Glance

### The Problem
```
Same type needs different presentations:
- Human reading text: "Enter server name:"
- TUI widget: Pretty box with styling
- AI agent: JSON schema for tool
```

### The Solution
```
type Style: Elicitation + Default + Clone + Send + Sync + 'static

Every type has a Style enum that controls HOW prompts are presented.
Style enum itself is elicitable (recursive).
```

### The Architecture
```
Communicator
    ↓ carries
StyleContext (HashMap<TypeId, Box<Style>>)
    ↓ accessed via
style_or_default()     → Use pre-set or fallback
style_or_elicit()      → Use pre-set or interactively choose
with_style<T, S>()     → Create new communicator with style

    ↓ feeds into
Elicitation::elicit(communicator)
    ↓ calls
ElicitationStyle::prompt_for_field()
    ↓ formats
Prompts for user/agent interaction
```

---

## 🏗️ Architecture Overview

```
┌──────────────────────────────────────────┐
│ User Type (e.g., Config, PlayerAction)   │
│ implements Elicitation                   │
│   type Style = ConfigStyle               │
└───────────────┬──────────────────────────┘
                │
    ┌───────────┴─────────────┐
    ▼                         ▼
ConfigStyle::Default    ConfigStyle::Compact
impl Elicitation        impl Elicitation
  (for style itself)      (for style itself)

    Both implement ElicitationStyle
              │
    ┌─────────┴──────────┐
    ▼                    ▼
prompt_for_field()   help_text()
validation_error()   show_type_hints()
select_style()       use_decorations()
prompt_prefix()
```

---

## 📋 File Locations Quick Reference

| What | File | Lines |
|------|------|-------|
| **Trait Definition** |
| Elicitation trait | traits.rs | 75-84 |
| Prompt trait | traits.rs | 43-50 |
| ElicitCommunicator | communicator.rs | 25-190 |
| **Storage** |
| StyleContext | communicator.rs | 192-245 |
| ElicitationContext | communicator.rs | 247-369 |
| **Implementations** |
| ElicitClient | client.rs | 40-222 |
| ElicitServer | server.rs | 33-141 |
| **Styles** |
| ElicitationStyle trait | style.rs | 40-158 |
| DefaultStyle | style.rs | 172-192 |
| CompactStyle | style.rs | 201-225 |
| VerboseStyle | style.rs | 234-278 |
| WizardStyle | style.rs | 287-337 |
| **Macro Generation** |
| Struct expansion | struct_impl.rs | Full file |
| Enum expansion | enum_impl.rs | Full file |
| Style enum gen (struct) | struct_impl.rs | 900-920 |
| Style enum gen (enum) | enum_impl.rs | 484-513 |
| **Paradigm Traits** |
| Select trait | paradigm.rs | 45-89 |
| Survey trait | paradigm.rs | 110-130 |
| Affirm trait | paradigm.rs | 91-108 |

---

## 💡 Key Insights

### 1. Separation of Concerns
- **WHAT** (Prompt trait): What question to ask
- **HOW** (Style trait): How to format the question
- Independent, composable concerns

### 2. Type-Safety
- `with_style::<T, S>()` ensures type safety at compile time
- Generic bounds prevent mismatches
- Compiler verifies correct Style for each Type

### 3. Recursive Elegance
- Style enums implement `Elicitation`
- Enables "style selection UI" using same mechanism as any type
- No special cases, no magic

### 4. Context Agnosticism
- Single implementation works for:
  - CLI text prompts
  - TUI widgets
  - AI agent MCP tools
  - Custom implementations

### 5. Performance
- O(1) lookup via TypeId
- Cheap cloning (Arc)
- No overhead if unused
- Zero feature flags

---

## 🔗 Cross-References

### Related Traits
- `Select` → Used for enum elicitation
- `Survey` → Used for struct elicitation
- `Affirm` → Used for bool elicitation
- `Prompt` → Metadata for prompts
- `ElicitIntrospect` → Type introspection
- `Generator` → Value generation (orthogonal to Style)

### Paradigm Patterns
- **Survey**: Multi-field (structs) - Sequential elicitation
- **Select**: Finite options (enums) - Choice elicitation
- **Affirm**: Binary (bool) - Yes/no confirmation
- **Primitive**: Direct value - Type-specific parsing

### Communication Modes
- **Server-side**: `peer.create_message()` to client (implemented)
- **Client-side**: `peer.call_tool()` (not yet implemented)
- **Extensible**: Any `ElicitCommunicator` impl works

---

## 🚀 Common Tasks

### Find where a trait is defined
→ QUICK_REF.md → FILES & LOCATIONS table

### Understand the flow for a type
→ QUICK_REF.md → FLOW section

### See a complete example
→ QUICK_REF.md → CONCRETE EXAMPLE: Blackjack

### Implement a custom style
→ DEEP_DIVE.md → Section 7
→ QUICK_REF.md → DESIGN PRINCIPLES section 1

### Understand macro generation
→ DEEP_DIVE.md → Section 3
→ QUICK_REF.md → MACRO GENERATION

### Debug style context issues
→ DEEP_DIVE.md → Section 2
→ QUICK_REF.md → STYLE CONTEXT

---

## 📖 Reading Guide

### For Architects (30 min)
1. RESEARCH.md - Get the big picture
2. DEEP_DIVE.md sections 1-2 - Understand core design
3. DEEP_DIVE.md section 8 - See the flow

### For Implementers (45 min)
1. QUICK_REF.md - Get the quick reference
2. DEEP_DIVE.md section 3 - Understand macro generation
3. QUICK_REF.md CONCRETE EXAMPLE - See a real case

### For Users (10 min)
1. QUICK_REF.md 5-MINUTE OVERVIEW
2. QUICK_REF.md CONCRETE EXAMPLE
3. QUICK_REF.md USAGE patterns

### For Researchers (Full)
1. Start with RESEARCH.md
2. Deep dive into DEEP_DIVE.md
3. Reference QUICK_REF.md as needed

---

## ✅ Verification Checklist

After reading these docs, you should be able to:

- [ ] Explain what Style is in one sentence
- [ ] Name all trait bounds on `type Style`
- [ ] Describe the difference between Prompt and Style
- [ ] Explain how StyleContext works internally
- [ ] Name the three key communicator methods for styles
- [ ] Describe how enums are elicitated (Select pattern)
- [ ] Describe how structs are elicitated (Survey pattern)
- [ ] Name all four built-in styles
- [ ] Trace the complete flow from user code to prompt
- [ ] Explain how macro generation works for styled structs
- [ ] Describe how TUI integration would work (future)
- [ ] Write a custom ElicitationStyle impl
- [ ] Apply a style to a struct via with_style()

---

## 📝 Notes

- **No TUI integration yet**: Architecture supports it, but no ratatui dependency
- **Client-side send_prompt not implemented**: Returns error in client.rs line 188
- **Server-side works**: Uses peer.create_message() in server.rs line 62
- **Macro generation is complex**: ~200 lines per pattern in derive macros
- **Style enums auto-generated**: Never manually implemented by users (usually)

---

## 🎓 Study Tips

1. **Start with the one-liner**: RESEARCH.md first paragraph
2. **Then the 5-minute overview**: QUICK_REF.md overview section
3. **Follow a concrete example**: QUICK_REF.md Blackjack example
4. **Trace the code**: Look at actual files referenced
5. **Draw diagrams**: Helps with recursive trait understanding
6. **Implement a custom style**: Best way to understand the system
7. **Experiment with macro output**: Use `cargo expand` to see generated code

---

Generated: 2024-03-11
Research depth: Comprehensive
Coverage: All core components
Examples: Multiple (primitives, structs, enums, games)
Type signatures: Complete
Files referenced: 15+
Lines of code analyzed: 2000+
