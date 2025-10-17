import '@testing-library/jest-dom';
import { vi } from 'vitest';

vi.mock("@tauri-apps/api/core", () => ({
    invoke: vi.fn(),
}));

vi.mock("@tauri-apps/api/dialog", () => ({
    open: vi.fn(),
}));