import { invoke } from "@tauri-apps/api/core";

import { RustOpenAIResponse, RustVoisiaMessage } from "../../types/dto";
import { OpenAIModelParams, TokenUsage } from "../../types/modelTypes";

/**
 * Get the full OpenAI API response object through Rust backend.
 * @param input - Array of conversation messages representing the dialogue history.
 * @param modelParams - Configuration parameters for the model behavior.
 * @returns A Promise resolving to the full OpenAI API response.
 */
export async function generateOpenAIResponse(
  input: RustVoisiaMessage | RustVoisiaMessage[],
  modelParams: OpenAIModelParams
): Promise<RustOpenAIResponse> {
  // Convert to an array if input is not already an array
  const messages = Array.isArray(input) ? input : [input];

  const lastMessage = messages[messages.length - 1];

  // Conversation history (excluding the last message)
  const conversationHistory: RustVoisiaMessage[] = messages.slice(0, -1).map((msg) => ({
    role: msg.role,
    content: msg.content,
  }));

  // Tauri invoke parameters
  const invokeParams = {
    model: modelParams.model,
    input: lastMessage.content,
    maxTokens: modelParams.maxOutputTokens, // Tauri converts to max_tokens
    temperature: modelParams.temperature,
    topP: modelParams.topP, // Tauri converts to top_p
    store: true,
    system: modelParams.instructions || null, // This will map to 'instructions' in Rust
    conversationHistory, // Tauri converts to conversation_history
  };

  const response: RustOpenAIResponse = await invoke("generate_openai_response", invokeParams);

  return response;
  /**
   * OpenAI Response Example:
   * {
   *   "id": "resp_67ccd3a9da748190baa7f1570fe91ac604becb25c45c1d41",
   *   "object": "response",
   *   "created_at": 1741476777,
   *   "status": "completed",
   *   "error": null,
   *   "incomplete_details": null,
   *   "instructions": null,
   *   "max_output_tokens": null,
   *   "model": "gpt-4o-2024-08-06",
   *   "output": [
   *     {
   *       "type": "message",
   *       "id": "msg_67ccd3acc8d48190a77525dc6de64b4104becb25c45c1d41",
   *       "status": "completed",
   *       "role": "assistant",
   *       "content": [
   *         {
   *           "type": "output_text",
   *           "text": "Hello! My name is OpenAI.",
   *           "annotations": []
   *         }
   *       ]
   *     }
   *   ],
   *   "parallel_tool_calls": true,
   *   "previous_response_id": null,
   *   "reasoning": {
   *     "effort": null,
   *     "summary": null
   *   },
   *   "store": true,
   *   "temperature": 1,
   *   "text": {
   *     "format": {
   *       "type": "text"
   *     }
   *   },
   *   "tool_choice": "auto",
   *   "tools": [],
   *   "top_p": 1,
   *   "truncation": "disabled",
   *   "usage": {
   *     "input_tokens": 328,
   *     "input_tokens_details": {
   *       "cached_tokens": 0
   *     },
   *     "output_tokens": 52,
   *     "output_tokens_details": {
   *       "reasoning_tokens": 0
   *     },
   *     "total_tokens": 380
   *   },
   *   "user": null,
   *   "metadata": {}
   * }
   */
}

/**
 * Extract role and content from OpenAI API response and create a VoisiaMessage.
 * @param response - The full OpenAI API response object.
 * @returns Simplified object with role and content only.
 */
export function createVoisiaMessage(response: RustOpenAIResponse): RustVoisiaMessage {
  // Extract content from the first choice
  const choice = response.choices[0];
  if (!choice) {
    throw new Error("No response choices available");
  }

  return {
    role: choice.message.role,
    content: choice.message.content,
  };
}

/**
 * Extract token usage information from an OpenAI API response.
 * @param response - The OpenAI API response object.
 * @returns Token usage information or null if unavailable.
 */
export function extractOpenAITokenUsage(response: RustOpenAIResponse): TokenUsage | null {
  if (response?.usage) {
    return {
      inputTokens: response.usage.prompt_tokens || 0,
      outputTokens: response.usage.completion_tokens || 0,
    };
  }
  return null;
}
