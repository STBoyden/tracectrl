import { CommandDialog } from "cmdk";
import { ModeToggle } from "./mode-toggle";
import { Command, CommandInput, CommandItem, CommandList } from "./ui/command";
import { useEffect, useState } from "react";

export function HeaderBar() {
  const [open, setOpen] = useState(false);
  const [search, setSearch] = useState("");

  useEffect(() => {
    const down = (e: KeyboardEvent) => {
      if (e.key === "k" && (e.metaKey || e.ctrlKey)) {
        e.preventDefault();
        setOpen((open) => !open);
      }
    };
    document.addEventListener("keydown", down);
    return () => document.removeEventListener("keydown", down);
  }, []);

  return (
    <div className="p-4 flex gap-1">
      <ModeToggle />

      {open && (
        <CommandDialog
          open={open}
          onOpenChange={setOpen}
          className="rounded-lg border shadow-md"
        >
          <CommandInput placeholder="Type a command or search..."></CommandInput>
          <CommandList>
            <CommandItem>Kill yourself</CommandItem>
          </CommandList>
        </CommandDialog>
      )}

      <Command onValueChange={() => setOpen(true)}>
        <CommandInput
          value={search}
          disabled={open}
          onValueChange={() => setOpen(true)}
          placeholder="Type a command or search..."
        ></CommandInput>
      </Command>
    </div>
  );
}
