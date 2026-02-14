import "@testing-library/jest-dom";
import { vi } from "vitest";

// Mock @tauri-apps/api/core
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

// Mock @tauri-apps/plugin-store
vi.mock("@tauri-apps/plugin-store", () => ({
  Store: vi.fn().mockImplementation(() => ({
    get: vi.fn(),
    set: vi.fn(),
    save: vi.fn(),
    delete: vi.fn(),
  })),
}));

// Mock @tauri-apps/plugin-notification
vi.mock("@tauri-apps/plugin-notification", () => ({
  sendNotification: vi.fn(),
  isPermissionGranted: vi.fn().mockResolvedValue(true),
  requestPermission: vi.fn().mockResolvedValue("granted"),
}));
