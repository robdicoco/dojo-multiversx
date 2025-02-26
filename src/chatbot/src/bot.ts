import axios from "axios";
import * as dotenv from "dotenv";

dotenv.config();

const OPENAI_API_KEY = process.env.OPENAI_API_KEY;
const OPENAI_API_URL =
    "https://api.openai.com/v1/engines/text-davinci-003/completions";

export const getBotResponse = async (userInput: string): Promise<string> => {
    try {
        const response = await axios.post(
            OPENAI_API_URL,
            {
                prompt: userInput,
                max_tokens: 50, // Limit the response length
            },
            {
                headers: {
                    "Content-Type": "application/json",
                    Authorization: `Bearer ${OPENAI_API_KEY}`,
                },
            }
        );

        return response.data.choices[0].text.trim();
    } catch (error) {
        console.error("Error fetching response from OpenAI:", error);
        return "Sorry, I encountered an error while processing your request.";
    }
};
