import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { invoke } from "@tauri-apps/api/core";
import LanguageSelect from "../../src/components/LanguageSelect";

const mockedInvoke = vi.mocked(invoke);

describe("LanguageSelect", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("displays all supported languages", () => {
    render(<LanguageSelect currentLanguage="KO" />);
    expect(screen.getByText("한국어 (KO)")).toBeInTheDocument();
    expect(screen.getByText("영어 (EN)")).toBeInTheDocument();
    expect(screen.getByText("일본어 (JA)")).toBeInTheDocument();
  });

  it("shows current language as selected", () => {
    render(<LanguageSelect currentLanguage="KO" />);
    const select = screen.getByRole("combobox");
    expect(select).toHaveValue("KO");
  });

  it("calls invoke on language change", async () => {
    mockedInvoke.mockResolvedValue(undefined);
    const onChange = vi.fn();

    render(<LanguageSelect currentLanguage="KO" onChange={onChange} />);
    const select = screen.getByRole("combobox");

    await userEvent.selectOptions(select, "EN");

    expect(mockedInvoke).toHaveBeenCalledWith("save_settings", {
      settings: { targetLanguage: "EN" },
    });
    expect(onChange).toHaveBeenCalledWith("EN");
  });
});
