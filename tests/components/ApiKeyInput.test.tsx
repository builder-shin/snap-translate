import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { invoke } from "@tauri-apps/api/core";
import ApiKeyInput from "../../src/components/ApiKeyInput";

const mockedInvoke = vi.mocked(invoke);

describe("ApiKeyInput", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("shows status when API key is set", () => {
    render(<ApiKeyInput hasApiKey={true} />);
    expect(screen.getByText("API Key가 설정되어 있습니다.")).toBeInTheDocument();
  });

  it("does not show status when API key is not set", () => {
    render(<ApiKeyInput hasApiKey={false} />);
    expect(screen.queryByText("API Key가 설정되어 있습니다.")).not.toBeInTheDocument();
  });

  it("saves API key on button click", async () => {
    mockedInvoke.mockResolvedValue(undefined);
    const onSaved = vi.fn();

    render(<ApiKeyInput hasApiKey={false} onSaved={onSaved} />);

    const input = screen.getByPlaceholderText("API Key 입력...");
    const button = screen.getByText("저장");

    await userEvent.type(input, "test-key:fx");
    await userEvent.click(button);

    await waitFor(() => {
      expect(mockedInvoke).toHaveBeenCalledWith("save_api_key", {
        apiKey: "test-key:fx",
      });
    });

    await waitFor(() => {
      expect(screen.getByText("API Key가 설정되었습니다.")).toBeInTheDocument();
    });
  });

  it("shows error on validation failure", async () => {
    mockedInvoke.mockRejectedValue("유효하지 않은 API Key입니다");

    render(<ApiKeyInput hasApiKey={false} />);

    const input = screen.getByPlaceholderText("API Key 입력...");
    await userEvent.type(input, "bad-key");
    await userEvent.click(screen.getByText("저장"));

    await waitFor(() => {
      expect(screen.getByText("유효하지 않은 API Key입니다")).toBeInTheDocument();
    });
  });

  it("masks API key input", () => {
    render(<ApiKeyInput hasApiKey={false} />);
    const input = screen.getByPlaceholderText("API Key 입력...");
    expect(input).toHaveAttribute("type", "password");
  });

  it("disables button when input is empty", () => {
    render(<ApiKeyInput hasApiKey={false} />);
    const button = screen.getByText("저장");
    expect(button).toBeDisabled();
  });
});
