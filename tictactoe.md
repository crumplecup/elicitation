# Proof-of-Concept: Verified Game Server

## Objective

Demonstrate that the elicitation framework enables developers to expose Rust libraries as MCP tools that allow agents to operate inside a type-safe transition algebra, where:

- All domain values are constructed through elicitation
- All transitions are expressed as tools
- Contracts formally describe relationships between transitions
- Agents can only construct valid states
- The system guarantees legality of operations even when agents behave unpredictably or adversarially

## Design Principles

### Tool Everything
All domain operations available to a Rust developer are exposed as tools or elicitable constructions.

### Types Define Validity
Invalid domain states are unrepresentable.

### Contracts Define Transitions
Relationships between operations are encoded as type-level proofs.

### Composition Defines Interfaces
Servers expose composed operations rather than primitive tools.

### Agents Operate Inside a State Machine
The exposed tool surface forms a constrained transition graph.

## Domain Choice: Tic-Tac-Toe

Tic-Tac-Toe provides:

- A fully deterministic rule system
- Clear legality constraints
- Alternating turns
- Terminal states
- Minimal implementation complexity

This allows the POC to focus on elicitation and contracts rather than game logic.

## Domain Model

All types derive:
- `Serialize`
- `Deserialize`
- `JsonSchema`
- `Elicit`

Example types:
- `Player`
- `Square`
- `Mark`
- `Board`
- `Move`
- `GameState`
- `GameResult`

`GameState` is the central transition object:

```rust
GameState {
    board: Board,
    current_player: Player,
    phase: Phase
}
```

`Phase` is a typestate enum:
- `AwaitingMove`
- `GameOver`

## Primitive Operations

Primitive tools represent atomic transitions:

- `place_mark_unchecked`
- `check_win_condition`
- `switch_player`
- `detect_draw`

These tools are **not exposed directly** to agents. They form the internal transition algebra.

## Contracts

Contracts encode legality:

**Examples:**
- `SquareIsEmpty`
- `MoveIsInBounds`
- `GameIsNotOver`
- `TurnBelongsToPlayer`

**Composite propositions:**

```rust
LegalMove = SquareIsEmpty AND MoveIsInBounds AND GameIsNotOver
```

**Transitions require proofs:**

```rust
apply_move(move, proof: Established<LegalMove>)
```

This ensures that invalid transitions cannot be constructed.

## Composed Operations (Agent-Visible)

The server exposes higher-level tools:

- `start_game()`
- `make_move()`
- `get_board()`

Internally, `make_move()` performs:

1. Elicit `Move`
2. Establish legality proofs
3. Apply transition
4. Evaluate terminal conditions
5. Produce next `GameState`

From the agent's perspective, only legal moves are possible.

## Interaction Modes

The server supports:

- Human vs Agent
- Agent vs Agent
- Agent Assist Mode

All modes use the same transition algebra.

## Demonstration Goals

The POC should visibly demonstrate:

- Agents cannot construct illegal moves
- All transitions are type-checked
- Game flow is enforced by typestate
- The exposed interface remains simple

## Logging and Observability

The server logs:

- Elicitation steps
- Established contracts
- State transitions

This provides an audit trail of verified execution.

## Extension Path

Future demonstrations may include:

- **Blackjack** (generators and hidden state)
- **Checkers** (larger transition algebra)
- **Workflow engines** (non-game domains)

These build on the same architecture without modifying elicitation primitives.

## Key Message

This POC demonstrates that:

> **Elicitation enables developers to expose domain libraries as verified conversational state machines, allowing agents to operate safely inside formally defined transition systems.**

## One Strategic Suggestion

If the goal of the vision doc is persuasion, the strongest framing is:

> **"We are not building better prompts. We are building type-safe operational semantics for agents."**

That communicates the ambition clearly to systems programmers.

---

*If you want, I can also propose a minimal crate layout for this POC (module structure, traits, and macro boundaries) that keeps the implementation small but architecturally clean.*
