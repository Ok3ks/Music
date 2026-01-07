from pydantic_ai import Agent
from typing import Literal
from dataclasses import dataclass

@dataclass
class OverallScore:
    rating: float  # 0-10
    summary: str

@dataclass
class PoeticExample:
    line: str
    technique: str
    effectiveness: str

@dataclass
class PoeticDevices:
    score: float  # 0-10
    strengths: list[str]
    examples: list[PoeticExample]
    improvements: list[str]

@dataclass
class NarrativeStructure:
    score: float  # 0-10
    arc_assessment: str
    strengths: list[str]
    improvements: list[str]

@dataclass
class EmotionalResonance:
    score: float  # 0-10
    primary_emotion: str
    show_vs_tell_ratio: str
    strengths: list[str]
    improvements: list[str]

@dataclass
class TechnicalCraft:
    score: float  # 0-10
    syllable_consistency: str
    singability: str
    flow_issues: list[str]
    strengths: list[str]

@dataclass
class Originality:
    score: float  # 0-10
    unique_elements: list[str]
    cliches: list[str]
    fresh_perspectives: list[str]

@dataclass
class ThematicCoherence:
    score: float  # 0-10
    central_theme: str
    focus_assessment: str
    improvements: list[str]

@dataclass
class PriorityRevision:
    area: str
    suggestion: str
    impact: Literal["high", "medium", "low"]

@dataclass
class LyricalAnalysis:
    overall_score: OverallScore
    poetic_devices: PoeticDevices
    narrative_structure: NarrativeStructure
    emotional_resonance: EmotionalResonance
    technical_craft: TechnicalCraft
    originality: Originality
    thematic_coherence: ThematicCoherence
    priority_revisions: list[PriorityRevision]