# Format of the hanzi.tsv file

The hanzi.tsv file is a tab-separated values (TSV) file that contains information about Chinese characters (hanzi). Each line in the file represents a single character and contains the following fields:
- Frequency: The frequency of the character in a given corpus, represented as an integer. This indicates how commonly the character is used. The 1 indicates the most common character.
- Character: The actual Simplified Chinese character.
- Traditional: The Traditional Chinese equivalent of the character, if applicable.
- Pinyin-with-tone: The pinyin representation of the character, which is the phonetic transcription in Mandarin Chinese.
- Pinyin-without-tone: The pinyin representation of the character without tone marks.
- Tone: The tone of the character, represented as an integer (1-4 for the four tones in Mandarin, with 5 representing the neutral tone).

# Format of hanzi_2.tsv file

The hanzi_2.tsv file is a converted version of the hanzi.tsv file. This file is created with `convert` command of this program. 

The first 6 fields of the each lines are the same as in the hanzi.tsv file:
- Frequency
- Character
- Traditional
- Pinyin-with-tone
- Pinyin-without-tone
- Tone

Aditionally, the hanzi_2.tsv file includes the following fields:
- Onset : The initial consonant sound of the character. If onset is not applicable (e.g., for vowel-initial characters), it is represented as "".
- Rime : The vowel sound of the character. 