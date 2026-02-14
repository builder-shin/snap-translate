import { describe, it, expect, vi, beforeEach } from "vitest";
import { invoke } from "@tauri-apps/api/core";
import { getSettings, saveApiKey, translate } from "../../src/lib/tauri";

const mockedInvoke = vi.mocked(invoke);

describe("Tauri API wrapper", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("getSettings calls invoke with correct command", async () => {
    const mockSettings = {
      hasApiKey: true,
      targetLanguage: "KO",
      hotkey: "CmdOrCtrl+Shift+D",
    };
    mockedInvoke.mockResolvedValue(mockSettings);

    const result = await getSettings();

    expect(mockedInvoke).toHaveBeenCalledWith("get_settings");
    expect(result).toEqual(mockSettings);
  });

  it("saveApiKey calls invoke with api key", async () => {
    mockedInvoke.mockResolvedValue(undefined);

    await saveApiKey("test-key:fx");

    expect(mockedInvoke).toHaveBeenCalledWith("save_api_key", {
      apiKey: "test-key:fx",
    });
  });

  it("translate calls invoke with correct params", async () => {
    const mockResult = { text: "안녕하세요", detectedSource: "EN" };
    mockedInvoke.mockResolvedValue(mockResult);

    const result = await translate("Hello", "KO");

    expect(mockedInvoke).toHaveBeenCalledWith("translate", {
      text: "Hello",
      targetLang: "KO",
      sourceLang: undefined,
    });
    expect(result).toEqual(mockResult);
  });
});
