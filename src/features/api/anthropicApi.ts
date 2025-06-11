import { invoke } from "@tauri-apps/api/core";

import { RustAnthropicResponse, RustVoisiaMessage } from "../../types/dto";
import { AnthropicModelParams, TokenUsage } from "../../types/modelTypes";

/**
 * Get the full Anthropic API response object through Rust backend.
 * @param input - Array of conversation messages representing the dialogue history.
 * @param modelParams - Configuration parameters for the model behavior.
 * @returns A Promise resolving to the full Anthropic API response.
 */
export async function generateAnthropicResponse(
  input: RustVoisiaMessage | RustVoisiaMessage[],
  modelParams: AnthropicModelParams
): Promise<RustAnthropicResponse> {
  // Convert to an array if input is not already an array
  const messages = Array.isArray(input) ? input : [input];

  // Get the last message (the user's new message)
  const lastMessage = messages[messages.length - 1];

  // Prepare conversation history (excluding the last message)
  const conversationHistory: RustVoisiaMessage[] = messages.slice(0, -1).map((msg) => ({
    role: msg.role,
    content: msg.content,
  }));

  // Tauri invoke parameters
  const invokeParams = {
    model: modelParams.model,
    input: lastMessage.content,
    system: modelParams.system || null,
    maxTokens: modelParams.maxOutputTokens, // Tauri converts to max_tokens
    temperature: modelParams.temperature,
    topP: modelParams.topP, // Tauri converts to top_p
    thinking: modelParams.thinking,
    convoHistory: conversationHistory, // Tauri converts to convo_history
  };

  // Call the get_anthropic_response command on the Rust side
  const response: RustAnthropicResponse = await invoke("generate_anthropic_response", invokeParams);

  return response;
  /**
   * Structure of the response:
   * {
   *   "content": [
   *     {
   *       "text": "Hi! My name is Claude.",
   *       "type": "text"
   *     }
   *   ],
   *   "id": "msg_013Zva2CMHLNnXjNJJKqJ2EF",
   *   "model": "claude-3-7-sonnet-20250219",
   *   "role": "assistant",
   *   "stop_reason": "end_turn",
   *   "stop_sequence": null,
   *   "type": "message",
   *   "usage": {
   *     "input_tokens": 2095,
   *     "output_tokens": 503
   *   }
   * }
   */
}

/**
 * Extract role and content from Anthropic API response and create a VoisiaMessage.
 * @param response - The full Anthropic API response object.
 * @returns Simplified object with role and content only.
 */
export function createVoisiaMessage(response: RustAnthropicResponse): RustVoisiaMessage {
  // Extract text content from response and join it
  const content = response.content
    .filter((block) => block.type === "text")
    .map((block) => block.text)
    .join("");

  return {
    role: response.role,
    content: content,
  };
}

/**
 * Extract token usage information from an Anthropic API response.
 * @param response - The Anthropic API response object.
 * @returns Token usage information or null if unavailable.
 */
export function extractAnthropicTokenUsage(response: RustAnthropicResponse): TokenUsage | null {
  if (response && response.usage) {
    return {
      inputTokens: response.usage.input_tokens || 0,
      outputTokens: response.usage.output_tokens || 0,
    };
  }
  return null;
}
