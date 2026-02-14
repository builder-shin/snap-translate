import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import HotkeyInput from "../../src/components/HotkeyInput";

describe("HotkeyInput", () => {
  it("displays hotkey parts as kbd elements", () => {
    render(<HotkeyInput currentHotkey="CmdOrCtrl+Shift+D" />);
    // The display should show individual key parts
    expect(screen.getByText("D")).toBeInTheDocument();
    expect(screen.getByText("Shift")).toBeInTheDocument();
  });

  it("shows future version hint", () => {
    render(<HotkeyInput currentHotkey="CmdOrCtrl+Shift+D" />);
    expect(
      screen.getByText("단축키 변경은 향후 버전에서 지원됩니다.")
    ).toBeInTheDocument();
  });
});
