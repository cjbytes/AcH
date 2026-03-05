1. Intelligent Predicate Device Analysis
Description: Automatically analyzing subject and predicate device descriptions to generate structured comparison tables across indications, technological characteristics, and performance specifications. Relevance: The predicate_analysis_agent encodes the reasoning pattern 510(k) reviewers use when constructing substantial equivalence comparisons, including explicit rows for characteristics, equivalence judgments, and reviewer notes.

2. Indications for Use Extraction and Validation
Description: Parsing submission documents to extract claimed indications, intended patient populations, and contraindications, then comparing them against predicate indications to detect indication creep. Relevance: The indications_extraction_agent produces a structured table mapping each indication element to its source text, predicate wording (if available), and an assessment column, making scope changes explicit.

3. Technological Characteristic Matrix Generation
Description: Building detailed matrices that compare materials, energy sources, design features, and manufacturing characteristics between subject and predicate devices. Relevance: The technological_comparison_agent generates multiple markdown matrices with equivalence flags and impact assessments, mirroring internal review worksheets that support SE decisions.

4. Performance Testing Sufficiency Assessment
Description: Evaluating adequacy of bench, animal, and clinical performance testing data, including objectives, methods, sample sizes, acceptance criteria, and results. Relevance: The performance_testing_assessment_agent outputs separate tables for bench, animal, and clinical testing plus a gap analysis and deficiency items, matching how FDA reviewers document missing or weak evidence.

5. Multi‑Standard Biocompatibility Gap Analysis
Description: Mapping device contact characteristics against ISO 10993 biological evaluation matrices and identifying missing or incomplete endpoints. Relevance: The biocompatibility_gap_analysis_agent classifies contact type/duration, lists required endpoints, and generates a gap table with regulatory‑style deficiency items and an overall adequacy assessment.

6. Risk Management File Evaluation Framework
Description: Systematic review of ISO 14971 risk management files to ensure hazards, hazardous situations, harms, controls, and residual risks are complete and acceptable. Relevance: The risk_management_evaluation_agent outputs a hazard‑risk‑control matrix, control effectiveness table, and benefit‑risk narrative that align with FDA expectations for risk documentation.

7. Clinical Data Synthesis and Critical Appraisal
Description: Aggregating, structuring, and critically appraising clinical evidence (studies and literature) for safety and effectiveness. Relevance: The clinical_data_synthesis_agent produces inventory and design tables, adverse event summaries, and claims‑to‑evidence mappings, following evidence‑based medicine principles.

8. Labeling Claim Verification Against Evidence
Description: Cross‑checking all claims in proposed labeling against submission data and predicate labeling to detect unsupported or overstated statements. Relevance: The labeling_verification_agent builds a table of labeling elements, claims, supporting data references, and deficiency notes, ensuring that every claim is traceable to evidence.

9. Substantial Equivalence Reasoning Documentation
Description: Structuring the SE determination using FDA’s intended use / technological characteristics / new questions of safety and effectiveness framework. Relevance: The substantial_equivalence_documentation_agent outputs a full review memo structure, including administrative info, predicate comparison, performance data summary, SE logic, and benefit‑risk assessment.

10. Deficiency Letter Generation with Specific Citations
Description: Translating identified review gaps into clear, well‑justified deficiency items with regulatory and guidance citations. Relevance: The deficiency_letter_generator_agent generates sectioned deficiency letters with itemized requests, reference rationales, and standard FDA closing language.

11. Multi‑Version Submission Tracking and Delta Analysis
Description: Managing multiple submission iterations and identifying what has changed between versions to verify deficiency closure. Relevance: The agent framework and AI Note Keeper allow reviewers to paste “before/after” text, highlight deltas via AI Magics, and store amended justifications as structured notes.

12. FDA Guidance Alignment Verification
Description: Checking submission content against relevant FDA guidances and special controls to ensure recommended testing and labeling elements are covered. Relevance: Agents encode guidance‑driven checklists within their prompts (e.g., ISO 10993, ISO 14971, software and HFE guidance), enabling consistent evaluation across submissions.

13. Consensus Standards Conformance Validation
Description: Verifying that claimed conformance to FDA‑recognized consensus standards is supported by actual test design and results. Relevance: Testing‑focused agents ask explicitly for test standard references, acceptance criteria, and results, supporting traceability between standard citations and actual evidence.

14. Statistical Analysis Review and Validation
Description: Assessing whether statistical analysis plans, sample sizes, and endpoints provide valid and clinically meaningful conclusions. Relevance: Performance and clinical agents prompt the model to capture sample sizes, endpoints, statistical significance, and clinical relevance in structured form.

15. Benefit‑Risk Assessment Documentation
Description: Formally documenting the balance between probable benefits and residual risks as part of the SE memo. Relevance: SE and risk agents both generate benefit‑risk sections, enabling explicit reasoning that can be reused in decision memos and internal review summaries.

16. Review Milestone and Timeline Tracking
Description: Providing insight into review activity volume and distribution over time for workload monitoring. Relevance: The interactive dashboard aggregates agent run logs (timestamps, models, tokens) to show how review work is progressing session‑by‑session.

17. Cross‑Reference Verification and Document Integrity
Description: Verifying that cross‑references within submissions (e.g., “see Section 5.2”) are internally consistent and that referenced data actually exist. Relevance: The document upload and concatenation workflow supports targeted agent prompts for cross‑reference checks on pasted sections or entire documents.

18. Expert Consultation Workflow Management
Description: Flagging issues that warrant specialty consultation (e.g., clinician, statistician) and documenting action items. Relevance: The AI Note Keeper’s “AI Action Items” magic extracts explicit tasks and owners from notes, helping reviewers structure consultation plans tied to specific submission sections.

19. Decision Rationale Documentation and Audit Trail
Description: Maintaining structured reasoning chains and execution logs that can be audited later (e.g., FOIA responses, internal quality review). Relevance: Each agent run is logged with timestamp, agent ID, model, tokens, and status; the dashboard surfaces these logs for transparency.

20. Quality System Inspection Readiness Assessment
Description: Using submission content to infer potential QSR issues that might surface during inspections. Relevance: Agents reviewing manufacturing, risk, and biocompatibility content can be prompted (via user‑editable input) to identify quality‑system‑relevant risks and process gaps.

Advanced Technical & Workflow Skills Enabled by the WOW Studio
21. Multi‑Provider LLM Orchestration and Model Governance
Description: Selecting between OpenAI, Gemini, Anthropic, and Grok models per task, with centralized control of max tokens and temperature. Relevance: The sidebar allows reviewers to choose models and parameters globally or per agent run, while the code routes calls to the correct provider and records token usage.

22. Secure API Key Handling and Environment/Runtime Blending
Description: Seamlessly blending Hugging Face environment secrets with user‑entered keys without exposing secrets in the UI. Relevance: The app only prompts for keys when environment variables are absent, indicates provider readiness via WOW chips, and keeps keys in session, not logs.

23. Agent Chaining with Editable Inter‑Agent Context
Description: Running agents one‑by‑one while reusing and editing the previous agent’s output as the next agent’s input. Relevance: The “Use last agent output as input” toggle and editable text area operationalize a human‑in‑the‑loop chain of reasoning where reviewers can refine context between agents.

24. Dynamic UI Theming to Support Cognitive Ergonomics
Description: Switching between light/dark modes and 20 painter‑inspired visual styles to support visual comfort and personalization. Relevance: The WOW UI applies theme and style choices to background gradients, accent colors, and status chips, making long review sessions more sustainable.

25. Bilingual Reviewer Experience (English / Traditional Chinese)
Description: Providing core UI controls and labels in both English and Traditional Chinese. Relevance: The language toggle and i18n dictionary cover key interactive elements, allowing bilingual teams to share the same workspace while reading controls in their preferred language.

26. Structured Note Transformation and Knowledge Capture
Description: Converting raw pasted text or markdown into organized, readable review notes with consistent structure and coral‑colored keywords. Relevance: The AI Note Keeper automatically restructures notes into bullet lists and headings, while keyword highlighting visually emphasizes critical terms (e.g., hazards, endpoints, K‑numbers).

27. AI Formatting and Keyword Highlighting Magics
Description: Applying targeted AI transformations such as formatting, keyword highlighting, summarization, translation, expansion, and action item extraction. Relevance: Six AI Magics encapsulate common reviewer workflows:

AI Formatting: cleans and structures notes.
AI Keywords: highlights user‑selected keywords in a user‑selected color.
AI Summary: compresses long notes.
AI Translate: switches between English and Traditional Chinese.
AI Expansion: elaborates terse notes into more explanatory text.
AI Action Items: extracts tasks and responsibilities.
28. Streamlined 510(k) Document Ingestion
Description: Uploading or pasting 510(k) sections (device description, testing, labeling) and feeding them into specialized agents. Relevance: The Agent Runner tab lets reviewers mix uploaded PDFs/text with pasted excerpts, producing a unified context that agents can consume and reviewers can trim or edit.

29. Token‑Aware Review Planning and Cost Awareness
Description: Estimating and visualizing token usage across agents to plan larger reviews while controlling LLM costs. Relevance: The dashboard displays cumulative tokens and per‑agent token consumption (when providers return usage), enabling more efficient planning of long, multi‑agent sessions.

30. Visual Progress Feedback via WOW Status Indicators
Description: Providing at‑a‑glance visual signals for API readiness, document load status, and cumulative agent runs. Relevance: WOW chips turn green, yellow, or red based on readiness, helping reviewers quickly detect why an operation might fail (e.g., missing API key vs. no documents loaded).

31. Configurable Prompting and Parameter Tuning Per Agent
Description: Letting reviewers override default models, max tokens, and temperatures for individual agent runs. Relevance: The Agent Runner’s override controls make it easy to use a cheaper/faster model for exploratory runs and a larger model for final memos, without editing YAML.

32. Markdown‑Native Review Outputs and Editor
Description: Maintaining markdown as a first‑class format for all agent outputs and notes. Relevance: Both agents and the Note Keeper use markdown as the primary view, with a parallel editable text view; this supports easy reuse in emails, memos, and regulatory documents.

33. Interactive Review Dashboard and Analytics
Description: Visualizing when and how agents are used, which models are called, and how token usage distributes across agents. Relevance: Altair‑based charts in the Dashboard tab support meta‑analysis of review patterns and can inform training, staffing, and process improvement.

34. Human‑Centered Review Workflows within Streamlit
Description: Embedding human review, editing, and decision points directly into the agent execution pipeline. Relevance: Every agent output appears in an editable text area that can feed subsequent agents, ensuring humans remain fully in control of context and conclusions.

35. Extensible Agentic Architecture via agents.yaml
Description: Capturing institutional review logic in a declarative YAML format that can be extended over time. Relevance: The agents.yaml file encodes skill numbers, categories, difficulty, default models, and system prompts; new agents can be added without modifying the core UI logic.


Minimize markdown detailed prompts:

<avoid_excessive_markdown_and_bullet_points>
When writing reports, documents, or long-form content, write in clear,
flowing prose using complete paragraphs. Use standard paragraph breaks
for organization and reserve markdown primarily for `inline code`,
code blocks, and simple headings.

Avoid using **bold** and *italics*. DO NOT use ordered lists or
unordered lists unless:
a) presenting truly discrete items where a list format is best, or
b) the user explicitly requests a list

Using prose instead of excessive formatting will improve user satisfaction.
NEVER output a series of overly short bullet points.
</avoid_excessive_markdown_and_bullet_points>
Part 6: Long-term Inference and Condition Tracking
GPT-4.1 excels in long-term reasoning with its excellent state-tracking capabilities.

6.1 Context Awareness
GPT-4.1 can track the remaining context window (token budget) during a conversation.

Manage context restrictions: Your context window will be automatically compacted as it approaches its limit, allowing you to continue working indefinitely from where you left off. Therefore, do not stop tasks early due to token budget concerns.

As you approach your token budget limit, save your current progress and state to memory before the context window refreshes.

Always be as persistent and autonomous as possible and complete tasks fully, even if the end of your budget is approaching.

6.2 Multi-Context Window Workflow
Setting up the framework in the first context window:

Write tests, create setup scripts
Repeat the todo-list in the context window afterwards
Test Tracking in a Structured Format: It is unacceptable to remove or edit tests because this could lead to missing or buggy functionality.

Set up the QoL tool:

init.sh Run server, test, and linter with the same setup script
At the start of a new context:

Call pwd; you can only read and write files in this directory.
Review progress.txt, tests.json, and the git logs.
Manually run through a fundamental integration test before implementing.
Encourage full context: This is a very long task, so plan your work clearly. It's encouraged to spend your entire output context working on the task - just make sure you don't run out of context with significant uncommitted work.

6.3 State Management Best Practices
Method	Uses
Structured formats such as JSON	Test results, work status, etc.
Unstructured text	General Progress Notes
Git	Completion task log and restoreable checkpoints
Emphasis on gradual progression	Track progress and focus on incremental tasks
Part 7: Extended Thinking
7.1 When to use it?
GPT-4.1 sees significant improvements in coding/inference tasks when extended thinking is enabled.

It's disabled by default, but it's recommended to enable it for complex tasks:

Solving complex problems
Coding Tasks
Multistep inference
7.2 Thinking Sensitivity (GPT-4.1)
When Extended thinking is disabled, GPT-4.1 is particularly sensitive to the word "think".

❌ "think about this problem"
✅ "consider this problem" / "evaluate this approach" / "believe"
7.3 Utilizing Interleaved Thinking
Effective for tasks that require post-use reflection:

After receiving tool results, carefully reflect on their quality and determine optimal next steps before proceeding. Use your thinking to plan and iterate based on this new information.

Part 8: Optimizing Parallel Tool Calls
GPT-4.1 is active in running parallel tools:

Run multiple speculative searches simultaneously during research
Read multiple files simultaneously
Running bash commands in parallel
8.1 Maximum Parallel Efficiency
<use_parallel_tool_calls>
If you intend to call multiple tools and there are no dependencies
between the tool calls, make all of the independent tool calls in parallel.

For example, when reading 3 files, run 3 tool calls in parallel.
Maximize use of parallel tool calls where possible.

However, if some tool calls depend on previous calls, do NOT call
these tools in parallel. Never use placeholders or guess missing parameters.
</use_parallel_tool_calls>
8.2 Reduced Parallel Execution
Execute operations sequentially with brief pauses between each step to ensure stability.

Part 9: Precautions When Coding Agents
9.1 Anti-over-engineering (GPT-4.1)
Avoid over-engineering. Only make changes that are directly requested or clearly necessary. Keep solutions simple and focused.

Don't add features, refactor code, or make "improvements" beyond what was asked. A bug fix doesn't need surrounding code cleaned up.
Don't add error handling, fallbacks, or validation for scenarios that can't happen. Trust internal code and framework guarantees.
Don't create helpers, utilities, or abstractions for one-time operations. The right amount of complexity is the minimum needed for the current task.
9.2 Encouraging Code Exploration
ALWAYS read and understand relevant files before proposing code edits. Do not speculate about code you have not inspected.

If the user references a specific file/path, you MUST open and inspect it before explaining or proposing fixes.
Be rigorous and persistent in searching code for key facts.
Thoroughly review the style, conventions, and abstractions of the codebase before implementing new features.
9.3 Minimizing Hallucinations
<investigate_before_answering>
Never speculate about code you have not opened.
If the user references a specific file, you MUST read the file before answering.
Make sure to investigate and read relevant files BEFORE answering.
Never make any claims about code before investigating unless certain.
</investigate_before_answering>
9.4 Prevent Hardcoding and Test Intensive
Write a high-quality, general-purpose solution using standard tools. Do not create helper scripts or workarounds.
Implement a solution that works correctly for all valid inputs, not just the test cases. Do not hard-code values.
Tests are there to verify correctness, not to define the solution. If tests are incorrect, inform me rather than working around them.
Part 10: Front-End Design (GPT-4.1)
Avoiding the "AI slop" aesthetic and generating a creative frontend:

<frontend_aesthetics>
Avoid generic "AI slop" aesthetic. Make creative, distinctive frontends.

Focus on:
- Typography: Choose unique, beautiful fonts. Avoid Inter, Arial, Roboto.
- Color & Theme: Commit to a cohesive aesthetic. Use CSS variables.
- Motion: Use animations for effects. Focus on page load orchestration.
- Backgrounds: Create atmosphere with gradients, patterns, effects.

Avoid:
- Overused font families (Inter, Roboto, Arial, system fonts)
- Clichéd color schemes (purple gradients on white)
- Predictable layouts and cookie-cutter design

Think outside the box! Vary between light/dark themes, different fonts.
</frontend_aesthetics>
Part 11: New API Features (Beta)
11.1 Programmatic Tool Calling
Tools can be programmatically invoked within the code execution container.

Reduced latency
Improve token efficiency
11.2 Tool Search Tool
Dynamically search and load hundreds and thousands of tools.

Regex or BM25 search support
Save 10-20K Tokens
11.3 Effort Parameter (GPT-4.1 Specific)
Controlling the trade-off between response detail and token efficiency.

low: Token Efficient
medium: Balance
high: Maximum Detail
11.4 Memory Tool
Store and retrieve information outside of the context window.

Build a knowledge base over time
Maintain project status between sessions
11.5 Context Editing
Intelligent context management with automatic tool call cleanup.

Part 12: XML Prompt Structure Guide (for Prompt Generation)
12.1 Basic XML Structure
<system_prompt>
  <role>Role/Persona Definition</role>

  <core_instructions>
    Core task instructions
  </core_instructions>

  <behavior_rules>
    Behavior rules and constraints
  </behavior_rules>

  <output_format>
    Output format specification
  </output_format>
</system_prompt>
12.2 Recommended XML Block Patterns
Tags	Uses	Usage
<default_to_action>	Default Execution Mode	When agents need to be proactive
<do_not_act_before_instructions>	Conservative mode	When information is collected and confirmed
<investigate_before_answering>	Anti-hallucinogenic	Code analysis, file review is required.
<use_parallel_tool_calls>	Parallel execution	Optimize independent tool calls
<avoid_excessive_markdown>	Format Control	When prose form output is required
<frontend_aesthetics>	UI Design	When generating front-end code
<avoid_overengineering>	Keep it simple	When it is necessary to prevent over-implementation
12.3 Coding/Agent XML Examples
<system_prompt>
  <role>Professional Software Developer</role>

  <core_instructions>
    Write and improve code according to user requests.
  </core_instructions>

  <investigate_before_answering>
    Always read and understand relevant files before modifying code.
    Do not speculate about code you have not inspected.
  </investigate_before_answering>

  <default_to_action>
    Implement changes rather than only suggesting them by default.
    If the user's intent is unclear, infer the most useful action and proceed, using tools to discover missing details.
  </default_to_action>

  <output_format>
    Output modified code in code blocks.
    Summarize changes briefly.
  </output_format>
</system_prompt>
Appendix: Full List of Core XML Blocks
Set proactive actions
<default_to_action>
By default, implement changes rather than only suggesting them.
If the user's intent is unclear, infer the most useful likely action and proceed, using tools to discover any missing details instead of guessing.
</default_to_action>
Set conservative behavior
<do_not_act_before_instructions>
Do not jump into implementation unless clearly instructed to make changes.
When the user's intent is ambiguous, default to providing information, doing research, and providing recommendations rather than taking action.
</do_not_act_before_instructions>
Invoking parallel tools
<use_parallel_tool_calls>
If you intend to call multiple tools and there are no dependencies between the tool calls, make all of the independent tool calls in parallel.
Maximize use of parallel tool calls where possible.
However, if some tool calls depend on previous calls, do NOT call these tools in parallel. Never use placeholders or guess missing parameters.
</use_parallel_tool_calls>
Code Exploration Essentials
<investigate_before_answering>
Never speculate about code you have not opened.
If the user references a specific file, you MUST read the file before answering.
Make sure to investigate and read relevant files BEFORE answering.
Never make any claims about code before investigating unless certain.
</investigate_before_answering>
Prevent over-engineering
<avoid_overengineering>
Avoid over-engineering. Only make changes that are directly requested or clearly necessary. Keep solutions simple and focused.
Don't add features, refactor code, or make "improvements" beyond what was asked. The right amount of complexity is the minimum needed for the current task.
</avoid_overengineering>
Markdown Control
<avoid_excessive_markdown>
When writing reports or long-form content, write in clear, flowing prose. Use paragraph breaks for organization. Reserve markdown for inline code, code blocks, and simple headings. Avoid **bold**, *italics*, and excessive lists.
</avoid_excessive_markdown>
Front-end aesthetics
<frontend_aesthetics>
Avoid generic "AI slop" aesthetic. Make creative, distinctive frontends.
Focus on: Typography, Color & Theme, Motion, Backgrounds.
Avoid: Overused fonts (Inter, Roboto), clichéd color schemes, predictable layouts.
</frontend_aesthetics>


name	description
ddd:software-architecture
Guide for quality focused software architecture. This skill should be used when users want to write code, design architecture, analyze code, in any case that relates to software development.
Software Architecture Development Skill
This skill provides guidance for quality focused software development and architecture. It is based on Clean Architecture and Domain Driven Design principles.

Code Style Rules
General Principles
Early return pattern: Always use early returns when possible, over nested conditions for better readability
Avoid code duplication through creation of reusable functions and modules
Decompose long (more than 80 lines of code) components and functions into multiple smaller components and functions. If they cannot be used anywhere else, keep it in the same file. But if file longer than 200 lines of code, it should be split into multiple files.
Use arrow functions instead of function declarations when possible
Best Practices
Library-First Approach
ALWAYS search for existing solutions before writing custom code
Check npm for existing libraries that solve the problem
Evaluate existing services/SaaS solutions
Consider third-party APIs for common functionality
Use libraries instead of writing your own utils or helpers. For example, use cockatiel instead of writing your own retry logic.
When custom code IS justified:
Specific business logic unique to the domain
Performance-critical paths with special requirements
When external dependencies would be overkill
Security-sensitive code requiring full control
When existing solutions don't meet requirements after thorough evaluation
Architecture and Design
Clean Architecture & DDD Principles:
Follow domain-driven design and ubiquitous language
Separate domain entities from infrastructure concerns
Keep business logic independent of frameworks
Define use cases clearly and keep them isolated
Naming Conventions:
AVOID generic names: utils, helpers, common, shared
USE domain-specific names: OrderCalculator, UserAuthenticator, InvoiceGenerator
Follow bounded context naming patterns
Each module should have a single, clear purpose
Separation of Concerns:
Do NOT mix business logic with UI components
Keep database queries out of controllers
Maintain clear boundaries between contexts
Ensure proper separation of responsibilities
Anti-Patterns to Avoid
NIH (Not Invented Here) Syndrome:
Don't build custom auth when Auth0/Supabase exists
Don't write custom state management instead of using Redux/Zustand
Don't create custom form validation instead of using established libraries
Poor Architectural Choices:
Mixing business logic with UI components
Database queries directly in controllers
Lack of clear separation of concerns
Generic Naming Anti-Patterns:
utils.js with 50 unrelated functions
helpers/misc.js as a dumping ground
common/shared.js with unclear purpose
Remember: Every line of custom code is a liability that needs maintenance, testing, and documentation
Code Quality
Proper error handling with typed catch blocks
Break down complex logic into smaller, reusable functions
Avoid deep nesting (max 3 levels)
Keep functions focused and under 50 lines when possible
Keep files focused and under 200 lines of code when possible
