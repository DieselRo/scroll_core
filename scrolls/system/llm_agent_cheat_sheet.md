---
title: llm agent cheat sheet
scroll_type: AgentCatalog
emotion_signature:
  tone: neutral
  emphasis: 0.0
  resonance: balanced
  intensity: 0.0
tags:
- imported
archetype: null
quorum_required: false
last_modified: null
file_path: null
---

LLM Agent Cheat Sheet ✍️

 Step #

 What you need to decide / build

 Key tips & gotchas

 1

Pick the workflow

Favour processes that are ambiguous, multi‑step, rely on unstructured data, or where existing rule engines are brittle.

 2

Choose the model(s)

Start with the most capable model (e.g. o3‑mini / GPT‑4o) to establish an accuracy baseline, then swap in cheaper models where quality remains ≥ target.

 3

Craft system instructions

One paragraph for role + numbered rules. Derive from existing SOPs; be explicit about required outputs & any edge‑case branches.

 4

Define tools

Break into Data, Action and Orchestration tools. 1 function = 1 clear verb.  Include JSON schema & crisp description.

 5

Guardrails & policy

Layer: relevance → safety → moderation → output validation. Mark high‑risk tools to require human approval.

 6

Choose orchestration pattern

Single‑agent loopManager (agents‑as‑tools)Decentralised hand‑offs Start simple & split when prompts/tools become unwieldy.

 7

Memory / Session storage

Lightweight: in‑memory or DB table keyed by app/user/session.  Advanced: RAG store or vector DB.

 8

Run‑loop implementation

While‑loop until: tool_call | final_output | error | max_turns.  Persist every Event to the Session log.

 9

Human‑in‑the‑loop

Escalate on exceeded retries or when high‑risk action requested.  Provide session context to the human.

 10

Instrumentation & evals

Track turn‑count, tool‑use, latency, success metric.  Add lightweight eval harness for new regressions.

How to wire the pieces together 🛠️

1.  Project skeleton

agents/              # python packages that define each agent class
  └─ my_agent/
       ├─ __init__.py
       ├─ agent.py    # LlmAgent + tools registration
       └─ requirements.txt
runners.py            # provided by ADK – manages sessions & invocation

2.  Register the tools

from google.genai import types
from adk.tools import FunctionTool

@get_weather = FunctionTool(
    name="get_weather",
    description="Return current conditions for a city",
    parameters={"city": {"type": "string"}}
)

3.  Build the agent

from adk.agents.llm_agent import LlmAgent
from adk.planners.built_in_planner import BuiltInPlanner
from adk.code_executors.unsafe_local_code_executor import UnsafeLocalCodeExecutor

weather_agent = LlmAgent(
    name="WeatherAgent",
    model="o3-mini",
    description="Answers weather questions",
    canonical_tools=[get_weather],
    planner=BuiltInPlanner(thinking_config={"max_thoughts":3}),
    code_executor=UnsafeLocalCodeExecutor(),
)

4.  Add guardrails

from adk.guardrails import RegexGuardrail, ModerationGuardrail

weather_agent.input_guardrails = [
    RegexGuardrail(patterns=[r"(?i)drop\s+table", r"SELECT .* FROM"], action="block"),
    ModerationGuardrail(),
]

5.  Create the run‑loop

from adk.runners import Runner

runner = Runner(
    app_name="weather_app",
    agent=weather_agent,
    session_service=InMemorySessionService(),
)

async for event in runner.run_async(user_id="u123", session_id="s1",
        new_message=types.Content(role="user", parts=[types.Part(text="Will it rain in Paris?")])):
    print(event.author, ":", event.content.parts[0].text)

6.  Escalation example

if event.actions.escalate:
    handoff_to_human(event, session)

7.  Observability hooks

Use RunConfig(streaming_mode=StreamingMode.TURN) and OpenTelemetry tracing built into ADK to measure latency, token usage & errors.

Drop this document into your repo’s /docs folder – it serves as a quick on‑boarding reference for any engineer touching the agent stack.