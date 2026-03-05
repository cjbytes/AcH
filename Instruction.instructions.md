📚 1️⃣ Intelligent Predicate Device Analysis
Agent: predicate_analysis_agent

Purpose: Auto‑generate structured comparison tables (subject vs predicate) for 510(k) submissions.
Output: Markdown matrix with rows for each characteristic, equivalence judgment, reviewer notes.
Key Prompt Elements: “Construct substantial‑equivalence rows”, “explicitly flag non‑equivalent items”, “include reviewer rationale”.
📚 2️⃣ Indications for Use Extraction & Validation
Agent: indications_extraction_agent

Purpose: Parse submission text, pull claimed indications, patient populations, contraindications, then cross‑check against the predicate.
Output: Table mapping Indication, Source Text, Predicate Wording (if any), Assessment (e.g., “Scope creep”, “Exact match”).
📚 3️⃣ Technological Characteristic Matrix Generation
Agent: technological_comparison_agent

Purpose: Build detailed side‑by‑side matrices of materials, energy sources, design & manufacturing features.
Output: Multiple markdown matrices with Equivalence Flag and Impact Assessment columns.
📚 4️⃣ Performance Testing Sufficiency Assessment
Agent: performance_testing_assessment_agent

Purpose: Evaluate bench, animal, and clinical testing data (objectives, methods, sample size, acceptance criteria, results).
Output: Three separate tables (bench, animal, clinical) + a Gap Analysis table listing deficiencies.
📚 5️⃣ Multi‑Standard Biocompatibility Gap Analysis
Agent: biocompatibility_gap_analysis_agent

Purpose: Map device contact type/duration to ISO 10993 required endpoints, flag missing or incomplete tests.
Output: Contact‑Type → Required Endpoint table with Gap column and an overall Adequacy rating.
📚 6️⃣ Risk Management File Evaluation Framework
Agent: risk_management_evaluation_agent

Purpose: Systematically review ISO 14971 risk files (hazards, hazardous situations, harms, controls, residual risk).
Output:
Hazard‑Risk‑Control matrix
Control effectiveness table
Benefit‑Risk narrative (free‑text).
📚 7️⃣ Clinical Data Synthesis & Critical Appraisal
Agent: clinical_data_synthesis_agent

Purpose: Aggregate all clinical evidence, structure it, and perform critical appraisal (bias, level of evidence).
Output:
Inventory table of studies (design, size, outcomes)
Adverse‑event summary table
Claims‑to‑Evidence mapping matrix.
📚 8️⃣ Labeling Claim Verification Against Evidence
Agent: labeling_verification_agent

Purpose: Cross‑check every labeling element (indications, warnings, instructions) with supporting data.
Output: Table of Label Element, Claim, Evidence Reference, Deficiency/Note.
📚 9️⃣ Substantial Equivalence Reasoning Documentation
Agent: substantial_equivalence_documentation_agent

Purpose: Produce a full SE memo using the FDA’s “intended use / technological characteristics / new questions” framework.
Output: Complete review memo sections: administrative info, predicate comparison, performance summary, SE logic, benefit‑risk assessment.
📚 🔟 Deficiency Letter Generation with Specific Citations
Agent: deficiency_letter_generator_agent

Purpose: Turn identified gaps into a regulator‑ready deficiency letter.
Output: Sectioned letter (e.g., “Section 1 – Biocompatibility”), each item includes Regulatory Citation (guidance, 21 CFR) and Requested Action.
📚 1️⃣1️⃣ Multi‑Version Submission Tracking & Delta Analysis
Agent: submission_delta_agent (uses AI Note Keeper & “AI Magics”)

Purpose: Compare two submission versions, highlight added/removed text, and store structured notes.
Output: Side‑by‑side diff table, Change Type (addition, deletion, modification), and Justification field.
📚 1️⃣2️⃣ FDA Guidance Alignment Verification
Agent: guidance_alignment_agent

Purpose: Verify that the submission covers all relevant FDA guidances (e.g., ISO 10993, ISO 14971, software, HFE).
Output: Checklist table with Guidance, Covered? (Y/N), Evidence Section, Comments.
📚 1️⃣3️⃣ Consensus Standards Conformance Validation
Agent: standards_conformance_agent

Purpose: Ensure claimed conformity to FDA‑recognized consensus standards is backed by test design & results.
Output: Table of Standard, Test Design Reference, Acceptance Criteria, Result, Pass/Fail.
📚 1️⃣4️⃣ Statistical Analysis Review & Validation
Agent: statistical_review_agent

Purpose: Assess statistical analysis plans, sample sizes, endpoints, and significance levels.
Output: Structured table with Study, Planned Sample Size, Stat Test, α Level, Power, Conclusion.
📚 1️⃣5️⃣ Benefit‑Risk Assessment Documentation
Agent: benefit_risk_agent

Purpose: Draft a formal benefit‑risk section for SE memoranda.
Output: Narrative plus a Benefit‑Risk Matrix (Benefit score vs Residual Risk score).
📚 1️⃣6️⃣ Review Milestone & Timeline Tracking
Agent: review_milestone_agent

Purpose: Log review activity (agent runs, timestamps, token usage).
Output: Dashboard‑ready CSV/JSON showing Date, Agent, Model, Tokens Used, Status.
📚 1️⃣7️⃣ Cross‑Reference Verification & Document Integrity
Agent: crossref_verification_agent

Purpose: Validate internal references (“see Section 5.2”) and ensure referenced data exists.
Output: Table of Reference, Found?, Location, Notes.
📚 1️⃣8️⃣ Expert Consultation Workflow Management
Agent: expert_consultation_agent

Purpose: Flag items that need specialist input (clinician, statistician, etc.) and generate action items.
Output: List of Issue, Recommended Expert, Owner, Due Date.
📚 1️⃣9️⃣ Decision Rationale Documentation & Audit Trail
Agent: audit_trail_agent

Purpose: Capture full reasoning chain for each decision, with execution logs for FOIA or internal QA.
Output: Chronological log (timestamp, agent, prompt, response) plus a Decision Summary paragraph.
📚 2️⃣0️⃣ Quality System Inspection Readiness Assessment
Agent: qsr_readiness_agent

Purpose: Infer potential QSR (Quality System Regulation) findings from submission content.
Output: Checklist of Potential Issue, Reference Section, Suggested Mitigation.
📚 2️⃣1️⃣ Multi‑Provider LLM Orchestration & Model Governance
Agent: model_governance_agent (sidebar configuration)

Purpose: Route each agent to the appropriate provider (OpenAI, Gemini, Anthropic, Grok) with per‑agent defaults for max tokens & temperature.
Output: Runtime log entry: Agent → Provider, Parameters, Token Count.
📚 2️⃣2️⃣ Secure API Key Handling & Environment Blending
Feature: api_key_manager

Purpose: Prompt for missing keys, merge with existing env vars, never log keys.
Behaviour: Shows provider readiness chip (green = key present).
📚 2️⃣3️⃣ Agent Chaining with Editable Inter‑Agent Context
Feature: chain_toggle

Purpose: Re‑use previous agent output as editable input for the next step.
Workflow: “Use last agent output as input” → edit → run next agent.
📚 2️⃣4️⃣ Dynamic UI Theming for Cognitive Ergonomics
Feature: theme_selector

Options: Light, Dark, 20 painter‑inspired palettes.
Effect: Adjust background gradients, accent colors, status‑chip hues.
📚 2️⃣5️⃣ Bilingual Reviewer Experience (EN / ZH‑TC)
Feature: i18n_toggle

Purpose: Switch all UI labels, tooltips, and help text between English and Traditional Chinese.
📚 2️⃣6️⃣ Structured Note Transformation & Knowledge Capture
Agent: note_keeper_agent

Purpose: Convert raw pasted text into structured markdown with coral‑colored keywords (hazards, endpoints, K‑numbers).
📚 2️⃣7️⃣ AI Formatting & Keyword Highlighting Magics
Magic	Action
AI Formatting	Clean & structure notes.
AI Keywords	Highlight chosen keywords in chosen color.
AI Summary	Produce concise summaries.
AI Translate	Switch between EN / ZH‑TC.
AI Expansion	Elaborate terse notes.
AI Action Items	Extract tasks & owners.
📚 2️⃣8️⃣ Streamlined 510(k) Document Ingestion
Feature: doc_ingest (Upload / Paste)

Supports: PDFs, plain‑text, markdown sections.
Result: Unified context passed to any agent.
📚 2️⃣9️⃣ Token‑Aware Review Planning & Cost Awareness
Feature: token_tracker

Shows: Cumulative tokens per agent, estimated cost (based on provider pricing).
📚 3️⃣0️⃣ Visual Progress Feedback via WOW Status Indicators
Feature: status_chips

Colors: Green = ready, Yellow = warning (missing doc), Red = error (no API key).
📚 3️⃣1️⃣ Configurable Prompting & Parameter Tuning Per Agent
Feature: agent_overrides

Allows: Override model, max tokens, temperature per run without editing YAML.
📚 3️⃣2️⃣ Markdown‑Native Review Outputs & Editor
Feature: markdown_editor

Dual Views: Rendered markdown & raw editable text.
Exports: .md, copy‑paste to emails, regulatory docs.
📚 3️⃣3️⃣ Interactive Review Dashboard & Analytics
Feature: dashboard_tab (Altair charts)

Shows: Agent usage over time, token distribution, model popularity.
📚 3️⃣4️⃣ Human‑Centered Review Workflows within Streamlit
Feature: human_in_the_loop

All outputs appear in editable text area, ready to be fed to the next agent.
📚 3️⃣5️⃣ Extensible Agentic Architecture via agents.yaml
File: agents.yaml

Contains: Skill ID, category, difficulty, default model, system prompt.
Additions: New agents can be defined without touching core UI code.
🛠️ NETWORK TOOLS (CLI)
Tool	Description	Core Commands	Primary Use‑Case
netflows	Extract TCP/UDP flows, resolve DNS from PCAPs.	netflows capture.pcap, netflows capture.pcap --source-ip 192.168.1.50, netflows capture.pcap --format json	IoT telemetry mapping, firewall rule creation.
iotnet	Detect IoT protocols, highlight security issues.	iotnet capture.pcap, sudo iotnet -i eth0 -d 30, iotnet capture.pcap --ip 192.168.1.10	Protocol inventory, vulnerability surfacing.
nmap	Professional network reconnaissance & port scanning.	See Default Two‑Phase Workflow below.	Service enumeration, version detection, vulnerability spotting.
Default Two‑Phase Nmap Workflow
#!/usr/bin/env bash
TARGET="192.168.1.100"
OUTPUT_DIR="./nmap-output"
mkdir -p "$OUTPUT_DIR"

# Phase 1 – Fast port discovery (SYN scan, requires sudo)
sudo nmap -p- "$TARGET" -oA "$OUTPUT_DIR/nmap-portscan"

# Retry if host appears down
if grep -q "Host seems down" "$OUTPUT_DIR/nmap-portscan.nmap"; then
    sudo nmap -p- -Pn "$TARGET" -oA "$OUTPUT_DIR/nmap-portscan"
fi

# Parse open ports
OPEN_PORTS=$(grep "^[0-9]" "$OUTPUT_DIR/nmap-portscan.nmap" |
            grep "open" |
            cut -d'/' -f1 |
            tr '\n' ',' |
            sed 's/,$//')

# Phase 2 – Service detection on open ports
if [ -n "$OPEN_PORTS" ]; then
    nmap -p "$OPEN_PORTS" -sV -sC "$TARGET" -oA "$OUTPUT_DIR/nmap-services"
else
    echo "No open ports found – skipping service detection."
fi

echo "Results saved in $OUTPUT_DIR"
Key Options (add as needed):

-T4 – faster timing.
-sU --top-ports 100 – UDP top‑100 scan.
-A – aggressive (OS, script, traceroute).
--script vuln – vulnerability NSE scripts.
-p <list> – target specific ports (e.g., 80,443,1883).
Best Practices

Always use -oA to generate .nmap, .xml, and .gnmap.
Run the two‑phase strategy unless a user explicitly asks for a different scan type.
Document every run (date, target, parameters, any anomalies).
Verify authorization before scanning any host.
📁 DIRECTORY STRUCTURE (example project)
/review‑studio
│
├─ agents.yaml               # Declarative agent catalog
├─ SKILLS.md                 # This file
├─ /netflows/                # netflows binary (or wrapper script)
│   └─ netflows
│
├─ /iotnet/                  # iotnet binary
│   └─ iotnet
│
├─ /nmap/                    # optional helper scripts
│   └─ run_nmap.sh           # wrapper for the default workflow
│
└─ /output/
    ├─ nmap-output/          # auto‑created by nmap runs
    ├─ netflows/             # JSON/text results from netflows
    └─ iotnet/               # JSON/text results from iotnet
🚀 Getting Started in VS Code
Clone or copy this repo into your workspace.
Install required binaries (netflows, iotnet, nmap).
Open the workspace in VS Code → use the Terminal to run any of the CLI tools.
Launch the Streamlit UI (if you have the Python front‑end) to access the AI agents, note‑keeper, and dashboard.
End of SKILLS.md


Okay but what about the SKILLS or /create-instructions that the AI can use while it's working to make it more efficient can you do a set of create-instructions for ACH.rs and beyond here's joel@P4C MINGW64 /c
$ cd RustProject

cjoel@P4C MINGW64 /c/RustProject (master)
$ ls -al
total 103
drwxr-xr-x 1 cjoel 197609 0 Mar 4 17:37 ./
drwxr-xr-x 1 cjoel 197609 0 Mar 4 14:40 ../
drwxr-xr-x 1 cjoel 197609 0 Mar 4 14:55 .git/
drwxr-xr-x 1 cjoel 197609 0 Mar 4 15:39 .github/
-rw-r--r-- 1 cjoel 197609 37 Mar 4 15:42 .gitignore
-rw-r--r-- 1 cjoel 197609 609 Mar 4 14:44 ACH.rs
drwxr-xr-x 1 cjoel 197609 0 Mar 4 16:44 ach_agent/
drwxr-xr-x 1 cjoel 197609 0 Mar 4 17:47 ach_ui/
-rw-r--r-- 1 cjoel 197609 1285 Mar 4 15:16 ach-agent-chat.prompt.md
-rw-r--r-- 1 cjoel 197609 169 Mar 4 17:37 Cargo.toml
-rw-r--r-- 1 cjoel 197609 40070 Mar 4 17:37 PLAN.txt
drwxr-xr-x 1 cjoel 197609 0 Mar 4 15:54 SKILLS/
-rw-r--r-- 1 cjoel 197609 28632 Mar 4 17:37 SKILLS.md
drwxr-xr-x 1 cjoel 197609 0 Mar 4 15:36 src/

cjoel@P4C MINGW64 /c/RustProject (master)
$
edit
Thought for a moment

Action: I will create a CREATE‑INSTRUCTIONS.md file that the AI agents (ACH‑agent, UI‑agent, etc.) can read at runtime.
The file contains:

General conventions – naming, logging, error handling.
Step‑by‑step instruction set for each component (ACH.rs, ach_agent/, ach_ui/).
Reusable snippets (Cargo commands, logging macros, JSON ↔ Markdown helpers).
Trigger phrases the LLM can watch for in the user’s prompts (“add a new command”, “refactor the parser”, etc.).
Below is the complete content you can drop into CREATE‑INSTRUCTIONS.md at the root of the repository.

CREATE‑INSTRUCTIONS.md
1️⃣ Overview
File / Folder	Primary Role	Who reads it	When used
ACH.rs	Core Rust library that implements the Automated Compliance Helper (ACH) logic.	ach_agent (LLM‑driven code generator)	Every time the agent creates, updates, or refactors Rust code.
ach_agent/	Backend service (Rust + optional Python glue) that receives natural‑language prompts, calls the LLM, and writes to the repo.	ach_agent (runtime)	On each incoming user request.
ach_ui/	Minimal web UI (React + Vite or Streamlit) that displays the ACH output and lets the user edit prompts.	ach_ui (front‑end)	When the UI renders or needs to show instructions.
SKILLS/	Collection of markdown skill definitions (e.g., SKILLS.md).	Human reviewers & other agents	Reference only.
CREATE‑INSTRUCTIONS.md	Canonical procedural guide for every agent.	All agents (read at start‑up)	Every run – ensures consistent behaviour.
2️⃣ Global Conventions
Convention	Description	Example
File‑header block	Include a short comment block at top of every source file with module name, purpose, and version.	//! ACH – Core compliance engine (v0.3.0)
Logging	Use log crate with env_logger. All messages start with [ACH] and a severity level.	log::info!("[ACH] Loaded predicate matrix – {} rows", rows);
Error handling	Return Result<T, anyhow::Error> for all public functions. Wrap external errors with context (anyhow::Context).	`fs::read_to_string(path).with_context(
JSON ↔ Markdown	Serialize structs to JSON for LLM exchange; deserialize Markdown tables using serde_json + comrak.	let md = comrak::markdown_to_html(&json_str, &options);
Naming	Rust snake_case for functions, PascalCase for types. UI: camelCase for props, kebab-case for files.	fn extract_predicates(...){} ; ComponentName.jsx
Testing	All modules get a tests sub‑module with #[cfg(test)]. Aim for ≥ 80 % line coverage.	cargo test -- --test-threads=1
Documentation	Every pub item must have a Rustdoc comment. UI components get JSDoc.	/// Returns a matrix of predicate equivalence.
Version bump	When ACH.rs changes, bump Cargo.toml patch (0.3.x → 0.3.y). Use cargo release patch.	
3️⃣ Instruction Sets
3.1 ACH.rs – Core Library
3.1.1 File Layout (exact order)
Header comment (see Global Conventions).
use statements – grouped: std, external crates, internal modules.
Public API – pub struct, pub fn definitions.
Internal helpers – fn (private).
Error types – #[derive(Debug, thiserror::Error)].
Tests – #[cfg(test)] mod tests { … }.
3.1.2 Core Functions (must exist)
Function	Signature	Purpose
load_predicate_matrix	fn load_predicate_matrix(path: &Path) -> Result<PredicateMatrix>	Parse CSV/JSON into a structured matrix.
compare_subject_to_predicate	fn compare_subject_to_predicate(subject: &Device, predicate: &Device) -> ComparisonResult	Produce a markdown table of equivalence rows.
extract_indications	fn extract_indications(doc: &str) -> Result<Vec<Indication>>	Pull indication blocks using regex + NLP heuristics.
biocompatibility_gap	fn biocompatibility_gap(device: &Device, standards: &Iso10993) -> GapReport	Generate ISO‑10993 gap matrix.
risk_assessment	fn risk_assessment(rmf: &RiskManagementFile) -> RiskReport	Build hazard‑risk‑control matrix.
write_markdown_report	fn write_markdown_report(report: &AchReport, out: &Path) -> Result<()>	Serialize AchReport (contains many sub‑reports) to a single markdown file.
All public functions return a named struct that implements serde::Serialize (for later JSON ↔ Markdown conversion).

3.1.3 Adding a New Feature
When the LLM receives a prompt like “Add a statistical‑analysis section to the report”:

Create a new module stat_analysis.rs under src/.
Add mod stat_analysis; to lib.rs.
Implement:
pub fn statistical_summary(studies: &[ClinicalStudy]) -> StatisticalReport {
    // 1️⃣ Validate inputs (sample size, α, power)
    // 2️⃣ Compute descriptive stats with `statrs` crate
    // 3️⃣ Build a markdown table and a JSON struct
}
Update AchReport with a new field: pub statistical: Option<StatisticalReport>.
Add a unit test in stat_analysis.rs.
Trigger phrase: add statistical → agent looks for the “stat_analysis” module template (provided below).

3.1.4 Snippet Library (copy‑paste)
/// Generic error wrapper for ACH operations
#[derive(Debug, thiserror::Error)]
pub enum AchError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("Missing field `{0}`")]
    MissingField(&'static str),
}
/// Helper: Convert a markdown table string to a JSON vector of rows
pub fn md_table_to_json(md: &str) -> anyhow::Result<serde_json::Value> {
    let parser = comrak::ComrakOptions::default();
    let html = comrak::markdown_to_html(md, &parser);
    // Use scraper crate to pull <tr>/<td> elements → Vec<Vec<String>>
    // Then serde_json::to_value(&vec_of_rows)
}
3.2 ach_agent/ – Backend Service
3.2.1 Startup Flow
Read CREATE‑INSTRUCTIONS.md (once, store in a global INSTR struct).
Initialize logger (env_logger::init();).
Load config from config.yaml (LLM provider, model, temperature, token budget).
Spin up HTTP server (actix‑web or warp). Endpoints:
Method	Path	Action
POST /prompt	Receive user prompt → call LLM → write to repo.	
GET /status	Return current token usage, last run timestamps.	
GET /log	Return tail of the operation log (for UI).	
Register middleware that injects the instruction set into every LLM request payload (system field).
3.2.2 Prompt Construction
let system_prompt = format!(
    "You are the ACH backend code generator. Follow the CREATE‑INSTRUCTIONS.md rules exactly.\n\
    When asked to add a feature, locate the appropriate Rust file, create or edit the module, \
    and always write unit tests. Return ONLY the **diff** in unified format, wrapped in triple backticks."
);
LLM response handling:

Expect a Markdown code‑block starting with diff.
Parse the diff, apply via git apply.
If git apply fails, return an error to the UI with the offending diff.
3.2.3 Logging Template (used everywhere)
[ACH-AGENT] <TIMESTAMP> | <LEVEL> | <ACTION> | <DETAILS>
Example:

[ACH-AGENT] 2026-03-05T14:07:22Z | INFO | PROMPT_RECEIVED | "Add biocompatibility gap analysis"
3.2.4 Common Helper Functions
/// Runs a shell command and returns stdout + stderr.
pub fn run_cmd(cmd: &str, args: &[&str]) -> anyhow::Result<String> { … }

/// Writes a diff to a temporary file, applies it, and returns git status.
pub fn apply_diff(diff: &str) -> anyhow::Result<()> { … }

/// Calculates token usage from LLM response header (if provider supplies it).
pub fn update_token_budget(used: usize) { /* deduct from config */ }
3.3 ach_ui/ – Front‑End
3.3.1 UI Components (React + Vite example)
Component	File	Purpose
PromptBox	src/components/PromptBox.jsx	Textarea + “Send” button.
LogViewer	src/components/LogViewer.jsx	Live tail of ach_agent logs (via SSE).
ReportViewer	src/components/ReportViewer.jsx	Render generated markdown (via marked lib).
DiffViewer	src/components/DiffViewer.jsx	Show diff returned by LLM; Accept / Reject buttons.
StatusBar	src/components/StatusBar.jsx	Token budget, last run time, UI theme toggle.
3.3.2 Interaction Flow
User types a prompt (e.g., “Add a risk‑management summary”).
PromptBox POSTs to /prompt.
ach_agent replies with a diff.
UI displays diff in DiffViewer.
User clicks “Accept” → UI sends POST /apply-diff (or re‑uses same endpoint).
Agent applies diff, runs cargo test.
If tests pass, UI fetches the updated markdown report (GET /report) and loads it into ReportViewer.
Errors (test failures, lint) are shown in LogViewer.
3.3.3 Theming & Language
Theme toggler updates a CSS variable --bg-primary.
UI text strings are stored in i18n/en.json and i18n/zh-TC.json.
The UI reads a lang query param (?lang=zh) and switches automatically.
3.3.4 Re‑usable Snippet (DiffViewer)
export function DiffViewer({diff, onAccept, onReject}) {
  return (
    <div className="diff-viewer">
      <pre className="diff">{diff}</pre>
      <button onClick={onAccept} className="btn btn-success">Accept</button>
      <button onClick={onReject} className="btn btn-danger">Reject</button>
    </div>
  );
}
4️⃣ Trigger Phrases & Agent “Intents”
Intent	Sample Prompt	What the LLM should do
ADD_MODULE	“Add a statistical‑analysis section”	Create src/stat_analysis.rs, add to lib.rs, update AchReport.
REFactor	“Refactor the predicate matrix loader to use async I/O”	Replace fs::read_to_string with tokio::fs::read_to_string, add #[tokio::main] to binary.
UPDATE_DOC	“Update the Rustdoc for compare_subject_to_predicate”	Edit the comment block directly above that function.
ADD_TEST	“Add unit tests for the new biocompatibility gap report”	Create a #[cfg(test)] module in biocompatibility.rs with at least 2 cases.
GENERATE_REPORT	“Generate a full compliance report for device XYZ”	Call ACH::write_markdown_report with dummy data; return the markdown path.
SET_THEME	“Switch UI to dark mode”	UI sends POST /theme → toggles CSS variable.
TOKEN_CHECK	“How many tokens have we used today?”	UI calls GET /status, displays remaining budget.
The agent must verify the intent by matching these keywords before constructing the diff.

5️⃣ Maintenance Checklist (run after each merge)
cargo fmt && cargo clippy -- -D warnings – enforce style.
cargo test --all --quiet – ensure green.
Update CHANGELOG.md (auto‑generated via git log --oneline).
Increment version in Cargo.toml.
Run npm run lint in ach_ui/.
Commit all changes with conventional commit message feat: / fix: / docs:.
6️⃣ Quick‑Start Script (optional)
Place this as scripts/setup.sh and run once after cloning:

#!/usr/bin/env bash
set -euo pipefail

# 1️⃣ Install Rust toolchain
rustup toolchain install stable
rustup default stable

# 2️⃣ Install UI dependencies
cd ach_ui
npm ci
cd ..

# 3️⃣ Install LLM provider SDKs (example: OpenAI)
pip install openai==1.2.0

# 4️⃣ Prepare env file
cat <<EOF > .env
OPENAI_API_KEY=your_key_here
TOKEN_BUDGET=500000   # per‑session token limit
EOF

# 5️⃣ Build & run backend (debug)
cargo run --bin ach_agent &
# 6️⃣ Start UI (development)
cd ach_ui && npm run dev &
echo "Setup complete – UI at http://localhost:3000, Agent at http://localhost:8000"
📌 How the LLM Uses This File
At start‑up the ach_agent reads CREATE‑INSTRUCTIONS.md into a string called INSTR.
When building a prompt for the LLM, it concatenates:
System Prompt = INSTR + "\n---\n" + USER_PROMPT
The LLM is forced to obey the rules (e.g., always return a diff, always include a unit test, never modify unrelated files).
If the model deviates, the agent rejects the output, logs LLM_VIOLATION, and asks the model to “please follow the CREATE‑INSTRUCTIONS exactly”.
That’s the complete instruction set for the ACH project.
Save the content as CREATE‑INSTRUCTIONS.md in the repository root, and all agents will automatically adopt the same disciplined workflow. Happy coding!


Ok now make this better for GPT 4.1 in Copilot less chat and more agent so tailor it to that and make x2 better
edit
Thought for a moment

CREATE‑INSTRUCTIONS.md – Optimised for GPT‑4.1 / Copilot‑style Agents

1️⃣ Goal
Make every agent (backend, UI, build‑scripts) act like a hands‑free code‑generator that:

Never asks the user for clarification – it infers intent from the prompt.
Returns only the exact artifact the model is asked to produce (diff, file, JSON, Markdown).
Validates the artifact automatically (git‑apply + cargo test).
Logs every step in a machine‑parsable line (JSON).
All rules below are enforced by the system prompt that GPT‑4.1 receives at the start of each request.

2️⃣ System Prompt (what the agent sees)
You are **ACH‑4‑agent**, an autonomous code‑writer that follows the
CREATE‑INSTRUCTIONS.md file verbatim.  Your job is to generate ONLY the
requested artifact – never extra prose, never explanations.

When a user request matches one of the “Intents” below, produce a **UNIFIED
DIFF** (git‑style) wrapped in a triple‑back‑tick block labelled “diff”.
If the request is to create a new file, output a diff that adds the file.
If the request is to modify existing code, output a diff that edits only
the necessary lines.

Always:

* Include a unit test for every new public function.
* Update the module‑tree (`mod …;`) if a new file is added.
* Bump the Cargo patch version (`0.x.Y → 0.x.(Y+1)`) in `Cargo.toml`.
* Add a line to `CHANGELOG.md` with the exact commit message you used.
* Log a JSON line to stdout after every successful operation:
  {"ts":"ISO‑8601","action":"apply_diff","status":"ok","files":["path"]}

If the produced diff cannot be applied cleanly, output **only**:

```diff
<error>
<explain why it fails in < 30 words>
</error>
Never ask the user for clarification. If you cannot infer a concrete implementation, reply with a single‑line JSON error:

{"error":"cannot_determine_intent","msg":"…"}
Use Rust 2021 edition, tokio for async, anyhow for errors, log + env_logger for logging, serde + serde_json for serialization.


---  

## 3️⃣  File‑Structure Rules (enforced on every diff)  

| Position | Rule |
|----------|------|
| **Header** | Top‑of‑file comment: `//! ACH – <module> (v0.x.Y)` |
| **Imports** | Grouped: `std`, external crates, internal modules. One blank line between groups. |
| **Public API** | All `pub` items first, each with a complete Rustdoc comment. |
| **Private Helpers** | After the public API, separated by a comment line `// ── Private helpers ──`. |
| **Error Types** | Single `enum AchError` in `error.rs` (or at bottom of file). |
| **Tests** | `#[cfg(test)] mod tests { … }` at bottom; at least one test per new function. |
| **Formatting** | Run `cargo fmt`; no `clippy` warnings allowed. |
| **Version** | After a successful diff, bump `patch` in `Cargo.toml`. |
| **Changelog** | Add a line: `- <date> <commit‑msg>`. |

---  

## 4️⃣  Intents & Trigger Patterns (what the model matches)  

| Intent | Trigger phrase (regex) | Produced artifact |
|--------|------------------------|-------------------|
| **ADD_MODULE** | `add\s+(statistical|risk|biocompatibility)\s+module` | Diff that creates `src/<name>.rs`, updates `mod.rs`. |
| **REFactor_IO** | `refactor\s+.*async|tokio` | Diff that replaces blocking I/O with `tokio::fs`. |
| **UPDATE_DOC** | `update\s+doc(umentation)?\s+for\s+(\w+)` | Diff that edits only the Rustdoc comment. |
| **ADD_TEST** | `add\s+unit\s+test\s+for\s+(\w+)` | Diff that appends a test function in the existing file. |
| **GENERATE_REPORT** | `generate\s+.*report\s+for\s+(\w+)` | Diff that adds a new `fn write_<name>_report` and a markdown template file. |
| **THEME_TOGGLE** | `set\s+ui\s+theme\s+(dark|light)` | UI‑only instruction (no Rust diff). Returns JSON `{ "theme":"dark" }`. |
| **TOKEN_STATUS** | `token\s+budget|usage` | JSON `{ "used":12345, "remaining":456789 }`. |
| **UNKNOWN** | any other | JSON error as defined above. |

The backend parses the user prompt, matches the first regex, and injects the matched groups into the *system prompt* (see §2) before calling GPT‑4.1.

---  

## 5️⃣  Diff Generation Template (copy‑paste for the model)  

```diff
diff --git a/src/<new_or_existing>.rs b/src/<new_or_existing>.rs
new file mode 100644
--- /dev/null
+++ b/src/<new_or_existing>.rs
@@
+//! ACH – <Module> (v0.x.Y)
+use anyhow::{Result, Context};
+use serde::{Serialize, Deserialize};
+
+/// <One‑sentence description of what this module does.>
+pub struct <StructName> { … }
+
+/// <Public function description>
+pub fn <fn_name>(args…) -> Result<ReturnType> {
+    // implementation
+}
+
+// ── Private helpers ──
+fn <helper>(…) { … }
+
+#[derive(Debug, thiserror::Error)]
+pub enum AchError { … }
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+    #[test]
+    fn test_<fn_name>() {
+        // simple assert
+    }
+}
The model fills only the + lines that are required.

6️⃣ Post‑Apply Automation (what the backend does)
Save diff to /tmp/diff.patch.
Run git apply /tmp/diff.patch.
If exit code ≠ 0 → send the <error> diff block back to the UI.
Run cargo test --quiet.
If failures → send the <error> diff block with the first failing test.
On success:
cargo fmt && cargo clippy -- -D warnings (fail → error block).
Increment patch version (cargo release patch -q).
Append changelog entry.
Emit JSON log line (see §2).
All steps are synchronous; the UI receives a single JSON response:

{
  "status":"ok",
  "files":["src/stat_analysis.rs","src/lib.rs"],
  "version":"0.3.2",
  "changelog":"- 2026‑03‑05 Add statistical‑analysis module"
}
7️⃣ Logging (machine‑readable)
Every operation writes one line to STDOUT (captured by the UI).

{
  "ts":"2026-03-05T14:07:22.123Z",
  "action":"apply_diff",
  "intent":"ADD_MODULE",
  "files":["src/stat_analysis.rs"],
  "status":"ok",
  "tokens_used":842
}
Errors use "status":"error" and include "error_code" and "msg" fields.

8️⃣ UI Contract (what the front‑end expects)
Endpoint	Method	Request body	Returns
/prompt	POST	{ "prompt":"…"}	JSON (see §6) OR a diff wrapped in diff if the backend needs user approval.
/apply	POST	{ "diff":"…"}	Same JSON as above (applies without further UI steps).
/status	GET	–	{ "tokens_used":…, "tokens_remaining":…, "last_run":"…"}
/log	GET	–	Stream of JSON log lines (newline‑delimited).
/theme	POST	{ "theme":"dark"}	{ "status":"ok"} . No diff is produced.
The UI never shows raw LLM prose – only the diff (if any) and the final JSON status.

9️⃣ Performance‑Boost Tips (x2 speed)
Tip	Why it helps
Batch prompts – send up to 5 user intents in one request (separate diff sections). GPT‑4.1 processes them in parallel internally, cutting round‑trip time.	
Cache static modules – store the generated header, use block, and test template in memory; LLM only receives the delta part.	
Token budget per request – set max_tokens = 350 (enough for a diff) to force short outputs and reduce latency.	
Parallel apply – if multiple diffs are returned, run git apply concurrently (POSIX &).	
Pre‑compiled regex – compile all intent patterns at startup; matching is O(1).	
Static analysis before LLM – run a quick heuristic (e.g., does the requested file already exist?). If yes, rewrite the prompt to “modify” instead of “add”; this avoids unnecessary file creation.	
10️⃣ Quick‑Start Script (Copilot‑ready)
#!/usr/bin/env bash
set -euo pipefail

# 1️⃣ install rust + deps
rustup toolchain install stable
cargo install cargo-release

# 2️⃣ install UI deps (Node 20+ assumed)
cd ach_ui && npm ci && cd ..

# 3️⃣ create env (GPT‑4.1 endpoint)
cat > .env <<EOF
OPENAI_API_KEY=YOUR_KEY
MODEL=gpt-4.1-turbo
TOKEN_BUDGET=800000
EOF

# 4️⃣ launch services (background)
cargo run --bin ach_agent &   # backend on :8000
cd ach_ui && npm run dev &    # UI on :3000

echo "🚀 ACH‑4 is live → UI http://localhost:3000"
📌 How to Use
Write a natural‑language request (e.g., “Add a statistical‑analysis module”).
The UI sends it to /prompt.
The backend matches the intent, builds the system prompt (§2), calls GPT‑4.1, receives a diff only and auto‑applies it.
If everything passes, the UI shows a green ✅ and the new markdown report.
All actions are logged in JSON, searchable for audits.