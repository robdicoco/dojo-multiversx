import express, { Request, Response } from "express";
import bodyParser from "body-parser";
import cors from "cors";
import fetch from "node-fetch";
import { Readable } from "stream";

const app = express();
const port = 3000;

// Middleware
app.use(cors());
app.use(bodyParser.json());
app.use(express.static("public"));

// Chat endpoint
app.post("/chat", async (req: Request, res: Response) => {
    try {
        const userMessage = req.body.message;

        if (!userMessage) {
            return res.status(400).json({ error: "Message is required" });
        }

        // Fetch response from Ollama server
        const response = await fetch("http://172.16.0.10:11434/api/generate", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({
                model: "deepseek-r1:1.5b",
                prompt: userMessage,
                stream: true,
            }),
        });

        if (!response.ok) {
            throw new Error(`HTTP error! Status: ${response.status}`);
        }

        if (!response.body) {
            throw new Error("No response body from Ollama");
        }

        let generatedText = "";

        // Create Node.js Readable stream with proper type assertion
        const readableStream = Readable.from(
            response.body as unknown as AsyncIterable<any>
        );

        readableStream.on("data", (chunk: Buffer) => {
            const chunkString = chunk.toString();
            const lines = chunkString.split("\n");

            for (const line of lines) {
                if (!line.trim()) continue;
                try {
                    const data = JSON.parse(line);
                    generatedText += data.response;
                } catch (err) {
                    console.error("Error parsing JSON:", err);
                }
            }
        });

        readableStream.on("end", () => {
            res.json({ response: generatedText });
        });

        readableStream.on("error", (err) => {
            console.error("Stream error:", err);
            res.status(500).json({ error: "Stream processing failed" });
        });
    } catch (error) {
        console.error("Error:", error);
        res.status(500).json({
            error: error instanceof Error ? error.message : "Unknown error",
        });
    }
});

// Start the server
app.listen(port, () => {
    console.log(`Server running at http://localhost:${port}`);
});
