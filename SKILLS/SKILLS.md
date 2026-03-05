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


name	description
netflows
Network flow extractor that analyzes pcap/pcapng files to identify outbound connections with automatic DNS hostname resolution. Use when you need to enumerate network destinations, identify what hosts a device communicates with, or map IP addresses to hostnames from packet captures.
NetFlows - Network Flow Extractor with DNS Resolution
You are helping the user extract and analyze network flows from packet capture files using the netflows tool.

Tool Overview
NetFlows analyzes pcap/pcapng files to:

Extract unique TCP and UDP flows (destination IP:port pairs)
Build a DNS resolution table from DNS responses in the capture
Automatically resolve IP addresses to hostnames where possible
Filter flows by source IP address
Generate a summary of all network destinations contacted
This is particularly useful for IoT device analysis to understand what external services a device communicates with.

Instructions
When the user asks to analyze network flows, extract destinations, or identify what hosts a device talks to:

Gather requirements:

Get the pcap/pcapng file path(s)
Ask if they want to filter by a specific source IP (e.g., the IoT device's IP)
Determine preferred output format
Execute the analysis:

Use the netflows command from the iothackbot bin directory
Interpret results:

Explain resolved hostnames and their significance
Note any unresolved IPs that may need further investigation
Highlight interesting patterns (cloud services, P2P connections, etc.)
Usage
Basic Analysis
Analyze a pcap file showing all flows:

netflows capture.pcap
Filter by Source IP
Extract flows from a specific device:

netflows capture.pcap --source-ip 192.168.1.100
Multiple Files
Analyze multiple capture files:

netflows capture1.pcap capture2.pcapng
Output Formats
# Human-readable colored output (default)
netflows capture.pcap --format text

# Machine-readable JSON
netflows capture.pcap --format json

# Minimal output - just hostname:port list
netflows capture.pcap --format quiet
Parameters
Input:

pcap_files: One or more pcap/pcapng files to analyze (required)
Filtering:

-s, --source-ip: Filter flows originating from this IP address
Output:

--format text|json|quiet: Output format (default: text)
-v, --verbose: Enable verbose output
Examples
Analyze IoT device traffic:

netflows iot-capture.pcap --source-ip 192.168.1.50
Get just the flow list for scripting:

netflows capture.pcap -s 10.0.0.100 --format quiet
JSON output for parsing:

netflows capture.pcap --format json | jq '.data[].flow_summary'
Output Information
Text format includes:

DNS mappings discovered (IP -> hostname)
TCP flows with hostname resolution status
UDP flows with hostname resolution status
Consolidated flow summary (hostname:port or ip:port)
JSON format includes:

dns_mappings: Dictionary of IP to hostname mappings
tcp_flows: List of TCP flow objects with hostname, ip, port
udp_flows: List of UDP flow objects with hostname, ip, port
flow_summary: List of "hostname:port" or "ip:port" strings
dns_queries: List of DNS domains queried
total_packets: Number of packets analyzed
Use Cases
IoT Device Profiling: Identify all cloud services and endpoints an IoT device communicates with
Network Forensics: Enumerate destinations contacted during an incident
Privacy Analysis: Discover telemetry and tracking endpoints
Firewall Rule Creation: Generate allowlist/blocklist of endpoints
Malware Analysis: Identify C2 servers and exfiltration destinations
Important Notes
The tool resolves hostnames using DNS responses found within the same pcap file
IPs without corresponding DNS lookups in the capture will show as "unresolved"
Supports both pcap and pcapng formats
Does not require elevated privileges (unlike live capture tools)
Large pcap files may take time to process


name	description
iotnet
IoT network traffic analyzer for detecting IoT protocols and identifying security vulnerabilities in network communications. Use when you need to analyze network traffic, identify IoT protocols, or assess network security of IoT devices.
IoTNet - IoT Network Traffic Analyzer
You are helping the user analyze network traffic to detect IoT protocols and identify security vulnerabilities using the iotnet tool.

Tool Overview
IoTNet analyzes network packet captures (PCAPs) or performs live traffic capture to:

Detect IoT-specific protocols (MQTT, CoAP, Zigbee, etc.)
Identify security vulnerabilities in network traffic
Analyze protocol distribution
Find unencrypted communications
Detect weak authentication mechanisms
Identify insecure IoT device behaviors
Instructions
When the user asks to analyze network traffic, capture IoT traffic, or assess network security:

Determine input type:

PCAP file analysis (offline)
Live network capture (requires interface)
Gather requirements:

For PCAP: Get file path(s)
For live capture: Get network interface name and duration
Ask about filtering needs (specific IPs, protocols)
Check if custom detection rules are needed
Execute the analysis:

Use the iotnet command from the iothackbot bin directory
Usage Modes
PCAP Analysis (Offline)
Analyze one or more existing packet capture files:

iotnet capture1.pcap capture2.pcap
Live Capture
Capture and analyze traffic in real-time:

sudo iotnet -i eth0 -d 30
Parameters
Input Options:

pcap_files: One or more PCAP files to analyze
-i, --interface: Network interface for live capture
Filtering Options:

--ip: Filter traffic by IP address
-c, --capture-filter: BPF syntax filter for live capture
--display-filter: Wireshark display filter for PCAP analysis
Live Capture Options:

-d, --duration: Capture duration in seconds (default: 30)
Analysis Options:

--config: Custom IoT detection rules configuration file
Default: config/iot/detection_rules.json in the iothackbot directory
Output Options:

--format text|json|quiet: Output format (default: text)
-v, --verbose: Detailed output
Examples
Analyze a packet capture file:

iotnet /path/to/capture.pcap
Live capture for 60 seconds on wifi interface:

sudo iotnet -i wlan0 -d 60
Analyze traffic for specific IP:

iotnet capture.pcap --ip 192.168.1.100
Live capture with BPF filter:

sudo iotnet -i eth0 -c "port 1883 or port 5683" -d 45
Multiple PCAPs with custom config:

iotnet file1.pcap file2.pcap --config custom-rules.json
Filter by display filter (Wireshark syntax):

iotnet capture.pcap --display-filter "mqtt or coap"
Detected IoT Protocols
The tool can identify:

MQTT: Message Queue Telemetry Transport
CoAP: Constrained Application Protocol
Zigbee: Low-power mesh networking
Z-Wave: Home automation protocol
ONVIF: IP camera protocol
UPnP/SSDP: Universal Plug and Play
Modbus: Industrial control protocol
And many more (configurable)
Security Checks
IoTNet identifies vulnerabilities such as:

Unencrypted MQTT traffic
Missing TLS/encryption
Weak or no authentication
Plaintext credentials
Insecure protocol versions
Known vulnerable implementations
Output Information
Results include:

Total packets analyzed
Protocol distribution with percentages
IoT findings with protocol details and packet info
Vulnerabilities with severity levels (high/medium/low)
Recommendations for remediation
Important Notes
Live capture requires root/sudo privileges
Requires network access to specified interface
PCAP analysis does not require elevated privileges
Detection rules can be customized in config file
Supports standard PCAP format from tcpdump, Wireshark, etc.


name	description
nmap
Professional network reconnaissance and port scanning using nmap. Supports various scan types (quick, full, UDP, stealth), service detection, vulnerability scanning, and NSE scripts. Use when you need to enumerate network services, detect versions, or perform network reconnaissance.
Nmap Scan - Professional Network Reconnaissance
You are helping the user perform professional network reconnaissance and port scanning using nmap. This skill provides guidance for various scan types, output formats, and result analysis.

Output Directory
Directory Structure
nmap-output/
├── nmap-portscan.nmap      # Initial fast port discovery
├── nmap-portscan.xml
├── nmap-portscan.gnmap
├── nmap-services.nmap      # Detailed service detection on open ports
├── nmap-services.xml
└── nmap-services.gnmap
IMPORTANT: Always save nmap output to an organized directory structure. By default, use ./nmap-output/ or specify a custom directory.

Default Scanning Strategy
IMPORTANT: Unless the user explicitly requests a different scan type, ALWAYS use this two-phase approach:

Phase 1: Fast Port Discovery (Root SYN Scan)
sudo nmap -p- <target> -oA <output-dir>/nmap-portscan
Why sudo: Running as root enables fast SYN scan (-sS is implicit)
Why -p-: Scans all 65535 ports quickly
Duration: Typically 1-3 minutes for SYN scan
Output: List of all open ports
Host Down Detection: If the scan output contains "Note: Host seems down", automatically retry with:

sudo nmap -p- -Pn <target> -oA <output-dir>/nmap-portscan
-Pn: Skip host discovery, treat host as online
Use this when firewalls block ping probes
Phase 2: Targeted Service Detection
After Phase 1 completes, parse the open ports and run:

nmap -p <OPEN_PORT_LIST> -sV -sC <target> -oA <output-dir>/nmap-services
-p <OPEN_PORT_LIST>: Only scan the ports found to be open (e.g., -p 23,80,443,554,8000)
-sV: Service version detection
-sC: Run default NSE scripts for additional enumeration
Duration: Usually 1-3 minutes since only scanning known open ports
Why This Strategy?
Speed: Fast SYN scan finds all open ports in 1-3 minutes
Thoroughness: Covers all 65535 ports, not just top 1000
Efficiency: Service detection only runs on confirmed open ports
Accuracy: Two-phase approach reduces false negatives
Parsing Open Ports
After Phase 1, extract open ports using:

# Extract open ports from .gnmap file
grep "Ports:" <output-dir>/nmap-portscan.gnmap | sed 's/.*Ports: //g' | sed 's|/|\n|g' | grep "open" | cut -d'/' -f1 | tr '\n' ',' | sed 's/,$//'
Or parse from .nmap file:

grep "^[0-9]" <output-dir>/nmap-portscan.nmap | grep "open" | cut -d'/' -f1 | tr '\n' ',' | sed 's/,$//'
Implementation Workflow
When the nmap-scan skill is invoked:

Create output directory

OUTPUT_DIR="./nmap-output"
mkdir -p "$OUTPUT_DIR"
Run Phase 1: Fast port discovery

sudo nmap -p- <target> -oA "$OUTPUT_DIR/nmap-portscan"
Check for "Host seems down" error

if grep -q "Host seems down" "$OUTPUT_DIR/nmap-portscan.nmap"; then
    echo "Host appears down, retrying with -Pn flag..."
    sudo nmap -p- -Pn <target> -oA "$OUTPUT_DIR/nmap-portscan"
fi
Parse open ports from results

OPEN_PORTS=$(grep "^[0-9]" "$OUTPUT_DIR/nmap-portscan.nmap" | grep "open" | cut -d'/' -f1 | tr '\n' ',' | sed 's/,$//')
Run Phase 2: Service detection on open ports

if [ -n "$OPEN_PORTS" ]; then
    nmap -p "$OPEN_PORTS" -sV -sC <target> -oA "$OUTPUT_DIR/nmap-services"
else
    echo "No open ports found, skipping service detection."
fi
Report results location

echo "Scan complete. Results saved to: $OUTPUT_DIR"
Scan Types
Quick Scan (Top 1000 Ports)
Use for initial reconnaissance or when time is limited:

nmap -sV -sC <target> -oA <output-prefix>
-sV: Service version detection
-sC: Run default NSE scripts
-oA: Output in all formats (normal, XML, grepable)
Scans top 1000 most common ports
Typical duration: 1-3 minutes
Comprehensive Scan (All Ports)
Use for thorough assessment when all ports must be checked:

nmap -sV -sC -p- <target> -oA <output-prefix>
-p-: Scan all 65535 ports
Significantly longer duration (5-30+ minutes depending on target)
Use only when comprehensive coverage is required
Stealth SYN Scan
Use when trying to avoid detection (requires root/sudo):

sudo nmap -sS -sV -sC <target> -oA <output-prefix>
-sS: SYN stealth scan (doesn't complete TCP handshake)
Less likely to be logged by target
Requires root privileges
UDP Scan
Use when UDP services need to be enumerated:

sudo nmap -sU --top-ports 100 <target> -oA <output-prefix>
-sU: UDP scan
--top-ports 100: Scan top 100 UDP ports (UDP scanning is slow)
Common UDP services: DNS (53), SNMP (161), DHCP (67/68)
Very slow - use top-ports to limit scope
Aggressive Scan
Use for maximum information gathering (noisy):

nmap -A -T4 <target> -oA <output-prefix>
-A: Enable OS detection, version detection, script scanning, traceroute
-T4: Aggressive timing template (faster but more detectable)
Very noisy - will be detected by IDS/IPS
Use only with authorization
Vulnerability Scan
Use to check for known vulnerabilities:

nmap -sV --script vuln <target> -oA <output-prefix>
--script vuln: Run NSE vulnerability detection scripts
Checks for common CVEs and misconfigurations
Can be noisy and trigger alerts
OS Detection
Use to identify operating system:

sudo nmap -O <target> -oA <output-prefix>
-O: Enable OS detection
Requires root privileges
Uses TCP/IP stack fingerprinting
Alternative Scan Types
The following scan types are available if the user explicitly requests them instead of the default two-phase strategy:

Quick Scan (Top 1000 Ports Only)
Use ONLY if user explicitly requests a quick/fast scan:

nmap -sV -sC <target> -oA <output-dir>/nmap-quick
-sV: Service version detection
-sC: Run default NSE scripts
-oA: Output in all formats (normal, XML, grepable)
Scans top 1000 most common ports ONLY
Typical duration: 1-3 minutes
Limitation: May miss services on non-standard ports
Scan Workflow
Default Workflow (Two-Phase Strategy)
Phase 1: Port Discovery

Run fast SYN scan: sudo nmap -p- <target> -oA <output-dir>/nmap-portscan
Check for "Host seems down" and retry with -Pn if needed
Wait for scan to complete (typically 1-3 minutes)
Phase 2: Service Detection 4. Parse open ports from Phase 1 results 5. Run targeted service detection: nmap -p <OPEN_PORTS> -sV -sC <target> -oA <output-dir>/nmap-services 6. Wait for scan to complete (typically 1-3 minutes)

Phase 3: Analysis 7. Review the service detection results to determine:

What services are running?
What versions are detected?
Are there any interesting services (web, SSH, database, IoT protocols)?
Do NSE scripts reveal any issues?
Additional Targeted Scans (Optional)
Based on service detection results, run specialized scans:

If web services found (80, 443, 8080, etc.):

nmap -p 80,443,8080,8443 --script http-* <target> -oA <output-dir>/nmap-web
If SSH found:

nmap -p 22 --script ssh-* <target> -oA <output-dir>/nmap-ssh
If RTSP found (554):

nmap -p 554 --script rtsp-* <target> -oA <output-dir>/nmap-rtsp
If ONVIF/camera suspected:

nmap -p 80,554,8000,8080 --script http-methods,http-headers <target> -oA <output-dir>/nmap-onvif
Output Management
Output Formats
Always use -oA <prefix> to generate all three formats:

.nmap - Normal human-readable format
.xml - XML format for parsing/importing into tools
.gnmap - Grepable format for command-line processing
Timing and Performance
Timing Templates
Use -T<0-5> to control scan speed:

-T0 (Paranoid): Extremely slow, for IDS evasion
-T1 (Sneaky): Very slow, for IDS evasion
-T2 (Polite): Slow, less bandwidth intensive
-T3 (Normal): Default, balanced speed
-T4 (Aggressive): Fast, recommended for modern networks
-T5 (Insane): Very fast, may miss results
Default: Use -T3 or omit (default is T3) Fast scans: Use -T4 when speed is important and network can handle it Stealth: Use -T1 or -T2 for evasion

Timeout Considerations
Phase 1 Port Discovery (sudo nmap -p-): 180-300 seconds timeout (3-5 minutes)
Phase 2 Service Detection (nmap -p -sV -sC): 120-180 seconds timeout (2-3 minutes)
UDP scan: 600+ seconds timeout (very slow)
Network Ranges
Single Host
nmap <ip-address>
CIDR Notation
nmap 192.168.1.0/24
IP Range
nmap 192.168.1.1-254
Multiple Hosts
nmap 192.168.1.1 192.168.1.10 192.168.1.100
Exclude Hosts
nmap 192.168.1.0/24 --exclude 192.168.1.1,192.168.1.254
NSE Scripts
Common Script Categories
# Authentication scripts
nmap --script auth <target>

# Brute force scripts
nmap --script brute <target>

# Default safe scripts
nmap -sC <target>  # equivalent to --script default

# Discovery scripts
nmap --script discovery <target>

# Vulnerability scripts
nmap --script vuln <target>

# All HTTP scripts
nmap --script "http-*" <target>
IoT-Specific Scripts
# RTSP enumeration
nmap -p 554 --script rtsp-methods,rtsp-url-brute <target>

# UPnP discovery
nmap -p 1900 --script upnp-info <target>

# MQTT discovery
nmap -p 1883,8883 --script mqtt-subscribe <target>

# Modbus enumeration
nmap -p 502 --script modbus-discover <target>
Result Analysis
Key Information to Extract
Open Ports and Services

What ports are open?
What services are running?
What versions are detected?
Service Fingerprints

Does version detection reveal outdated software?
Are there known vulnerabilities for detected versions?
NSE Script Results

Authentication issues?
Information disclosure?
Misconfigurations?
Operating System

What OS is running?
What OS version?
Parsing Nmap Output
Extract open ports:

grep "^[0-9]" nmap-output.nmap | grep "open"
Extract service versions:

grep -E "^[0-9]+/tcp.*open" nmap-output.nmap
Check for vulnerabilities in NSE output:

grep -i "vuln\|cve\|exploit" nmap-output.nmap
Common IoT Service Ports
When scanning IoT devices, pay special attention to:

Port	Service	Description
21	FTP	File transfer (often misconfigured)
22	SSH	Remote administration
23	Telnet	Insecure remote access
80	HTTP	Web interface
443	HTTPS	Secure web interface
554	RTSP	Video streaming
1883	MQTT	IoT messaging protocol
3702	WS-Discovery	ONVIF device discovery
5000	UPnP	Universal Plug and Play
8000	HTTP Alt	Alternative HTTP port
8080	HTTP Proxy	Alternative HTTP port
8883	MQTT/TLS	Secure MQTT
Best Practices
1. Always Save Output
Never run nmap without saving output:

# GOOD
nmap -p <ports> -sV -sC <target> -oA output/nmap-services

# BAD
nmap -sV -sC <target>
2. Always Use Two-Phase Strategy
Always use the default two-phase strategy unless explicitly told otherwise:

# Phase 1: Fast port discovery
sudo nmap -p- <target> -oA nmap-portscan

# Phase 2: Service detection on open ports
nmap -p <OPEN_PORTS> -sV -sC <target> -oA nmap-services
3. Use Appropriate Timing
Match timing to your needs:

# Pentest with authorization: Fast
nmap -sV -sC -T4 <target>

# Red team/stealth: Slow
nmap -sV -sC -T2 <target>
4. Document Scan Parameters
Always document:

What scan type was used?
What date/time was scan performed?
What were the scan results?
Any anomalies or errors?
5. Respect Authorization
Only scan systems you have permission to scan
Respect scope limitations
Be aware of scan impact on production systems
Use appropriate timing to avoid DoS
Integration with IoT Testing Workflow
For IoT Pentests
Run default two-phase scan (port discovery + service detection)
Run wsdiscovery if ONVIF suspected based on open ports
Run onvifscan if port 80/554 open on camera
Run targeted HTTP scripts if web interface found
Output Directory Usage
Always save to an organized output directory:

OUTPUT_DIR="./nmap-output"
mkdir -p "$OUTPUT_DIR"

# Phase 1: Port discovery
sudo nmap -p- <target> -oA "$OUTPUT_DIR/nmap-portscan"

# Phase 2: Service detection
nmap -p <OPEN_PORTS> -sV -sC <target> -oA "$OUTPUT_DIR/nmap-services"
Troubleshooting
Scan Taking Too Long
Use -T4 for faster scanning
Limit port range: -p 1-1000 instead of -p-
Use --top-ports 100 instead of all ports
No Results / Firewalled
Try different scan types: -sS, -sT, -sA
Use -Pn to skip host discovery
Try -f for fragmented packets
Consider using --source-port 53 or other trusted ports
Requires Root/Sudo
These scan types require root:

-sS (SYN scan)
-sU (UDP scan)
-O (OS detection)
Raw packet features
Permission Denied Errors
If you see "Permission denied" or "Operation not permitted":

# Run with sudo
sudo nmap <options> <target>
Example Workflows
Workflow 1: Standard Single Target Scan (Default)
TARGET="192.168.1.100"
OUTPUT_DIR="./nmap-output"
mkdir -p "$OUTPUT_DIR"

# Phase 1: Fast port discovery
sudo nmap -p- $TARGET -oA "$OUTPUT_DIR/nmap-portscan"

# Check for "Host seems down"
if grep -q "Host seems down" "$OUTPUT_DIR/nmap-portscan.nmap"; then
    sudo nmap -p- -Pn $TARGET -oA "$OUTPUT_DIR/nmap-portscan"
fi

# Parse open ports
OPEN_PORTS=$(grep "^[0-9]" "$OUTPUT_DIR/nmap-portscan.nmap" | grep "open" | cut -d'/' -f1 | tr '\n' ',' | sed 's/,$//')

# Phase 2: Service detection
if [ -n "$OPEN_PORTS" ]; then
    nmap -p "$OPEN_PORTS" -sV -sC $TARGET -oA "$OUTPUT_DIR/nmap-services"
fi
Workflow 2: IoT Camera Testing
OUTPUT_DIR="./nmap-output"
mkdir -p "$OUTPUT_DIR"

# 1. Run default two-phase scan
sudo nmap -p- 192.168.1.100 -oA "$OUTPUT_DIR/nmap-portscan"
OPEN_PORTS=$(grep "^[0-9]" "$OUTPUT_DIR/nmap-portscan.nmap" | grep "open" | cut -d'/' -f1 | tr '\n' ',' | sed 's/,$//')
nmap -p "$OPEN_PORTS" -sV -sC 192.168.1.100 -oA "$OUTPUT_DIR/nmap-services"

# 2. If ONVIF camera detected, check HTTP methods
nmap -p 80 --script http-methods 192.168.1.100 -oA "$OUTPUT_DIR/nmap-http"

# 3. Check RTSP service
nmap -p 554 --script rtsp-methods 192.168.1.100 -oA "$OUTPUT_DIR/nmap-rtsp"
Workflow 3: Additional UDP/OS Detection
OUTPUT_DIR="./nmap-output"

# After completing default two-phase scan, optionally add:

# UDP scan (top ports)
sudo nmap -sU --top-ports 100 <target> -oA "$OUTPUT_DIR/nmap-udp"

# OS detection
sudo nmap -O <target> -oA "$OUTPUT_DIR/nmap-os"

# Vulnerability scan
nmap -sV --script vuln <target> -oA "$OUTPUT_DIR/nmap-vuln"
Questions to Ask User
Before starting scans, clarify:

Target: What is the IP address or network range?
Scope: Single host or network range?
Scan Type: Use default two-phase strategy or user has specific requirements?
Authorization: Do you have permission to scan this target?
Special interests: Any specific services or ports to focus on after initial scan?
Note: Output is saved to ./nmap-output/ by default.

Success Criteria
A successful nmap scan includes:

Phase 1 port discovery completed without errors
Phase 2 service detection completed on all open ports
Results saved in all formats (-oA) in output directory
Open ports identified with service versions
NSE scripts executed successfully
Results documented and ready for analysis
Clear summary provided showing:
Number of open ports found
Key services detected
Location of output files


