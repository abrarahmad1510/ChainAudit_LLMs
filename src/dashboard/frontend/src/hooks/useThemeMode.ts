import { useLocalStorage } from './useLocalStorage';

type ThemeMode = 'light' | 'dark';

export function useThemeMode() {
  const [mode, setMode] = useLocalStorage<ThemeMode>('themeMode', 'light');

  const toggleMode = () => {
    setMode(mode === 'light' ? 'dark' : 'light');
  };

  return { mode, toggleMode };
}
