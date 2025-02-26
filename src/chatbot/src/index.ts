import express from "express";
import { getBotResponse } from "./bot";

const app = express();
const PORT = 3000;

// Middleware to parse JSON
app.use(express.json());

// Route to handle chatbot requests
app.post("/chat", async (req, res) => {
    const { message } = req.body;

    if (!message) {
        return res.status(400).json({ error: "Message is required" });
    }

    try {
        const botResponse = await getBotResponse(message);
        res.json({ response: botResponse });
    } catch (error) {
        res.status(500).json({ error: "Internal server error" });
    }
});

// Start the server
app.listen(PORT, () => {
    console.log(`Server is running on http://localhost:${PORT}`);
});
