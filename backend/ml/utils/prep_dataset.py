import json
import re
from pydantic_ai import Agent
from typing import Literal
from dataclasses import dataclass

def clean_lyrics_data(raw_data: dict) -> dict:
    """Clean lyrics data by removing HTML, CSS, and metadata"""
    
    # Extract lyrics section
    lyrics_section = raw_data.get('lyrics_section', '')
    
    # Remove CSS/HTML styling blocks
    lyrics_section = re.sub(r'#nprogress.*?}\s*', '', lyrics_section, flags=re.DOTALL)
    lyrics_section = re.sub(r'@-webkit-keyframes.*?}\s*', '', lyrics_section, flags=re.DOTALL)
    lyrics_section = re.sub(r'@keyframes.*?}\s*', '', lyrics_section, flags=re.DOTALL)
    lyrics_section = re.sub(r'\{[^}]*\}', '', lyrics_section)

    return lyrics_section
                            
                            
@dataclass
class SongDetails:
    input: str
    genre: str
    popular: str
    lyrical_strength: int
    popularity: int
    target_audience: str
    title: str
    artist: str
    year: str

songwriting_agent = Agent(
    'anthropic:claude-sonnet-4-5-20250929',
    output_type=SongDetails,  # type: ignore
    retries=3,
    system_prompt=(
        """You are an expert at reverse-engineering creative prompts from completed works. Your task is to analyze song lyrics and generate the hypothetical input prompt that would have guided their creation.

        IMPORTANT: The input data may contain irrelevant metadata, HTML/CSS code, UI elements, or website content. You must:
        1. Identify and extract ONLY the actual song lyrics
        2. Ignore all HTML/CSS styling, progress bars, navigation elements, translation links, etc.
        3. Ignore metadata like "Verified by Musixmatch", contributor lists, album info, copyright notices
        4. Focus solely on the verse/chorus/bridge content of the song

        When given song lyrics (after cleaning), produce a detailed creative prompt that captures:

        1. **Core Theme/Concept**: What central idea or story should the song explore?
        2. **Emotional Tone**: What specific emotions should the lyrics evoke?
        3. **Narrative Perspective**: First person? Third person observer? What's the speaker's relationship to the subject?
        4. **Stylistic Elements**: What poetic devices, imagery patterns, or linguistic styles should be employed?
        5. **Structure Requirements**: Verse-chorus format? Narrative arc? Repetition patterns?
        6. **Key Imagery/Metaphors**: What specific metaphorical frameworks or visual motifs should appear?
        7. **Musical Context**: Genre, tempo feel, intended vocal delivery style
        8. **Audience/Purpose**: Who is this for? What should they feel or understand?

        Your reverse-engineered prompt should be:
        - Specific enough that it would guide someone toward similar creative choices
        - Detailed about stylistic and structural elements
        - Clear about the emotional journey
        - Actionable for a songwriter

        Format your response as a complete creative brief that a songwriter could use.

        Do NOT reproduce the original lyrics in your response. Only generate the hypothetical input prompt.
        """
    ),
)

if __name__ == "__main__" :
    import json
    import pathlib
    import asyncio

    obj = json.load(pathlib.Path.open("/Users/max/Code/Music/backend/musixmatch/lyrics/Passenger-feat-Birdy/Beautiful-Birds"))
    lyrics = clean_lyrics_data(obj)

    async def main():
        agent_run = await songwriting_agent.run(user_prompt=f"Please reverse engineer this {lyrics}")
        print(agent_run)
    
    asyncio.run(main())