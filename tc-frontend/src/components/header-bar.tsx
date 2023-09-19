import { cn } from "@/lib/utils";
import { ModeToggle } from "./mode-toggle";
import {
  Command,
  CommandInput,
  CommandItem,
  CommandList,
  CommandDialog,
  CommandGroup,
  CommandEmpty,
} from "./ui/command";
import { useEffect, useState } from "react";
import { Button } from "./ui/button";
import { Cross1Icon } from "@radix-ui/react-icons";
import { Separator } from "./ui/separator";

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
    <>
      <div
        className={cn(
          "p-4",
          "mx-auto",
          "flex",
          "gap-1",
          "md:max-w-2xl",
          "lg:max-w-4xl"
        )}
      >
        <Command onValueChange={() => setOpen(true)}>
          <CommandInput
            value={search}
            onValueChange={(search: string) => {
              setSearch(search);
              setOpen(true);
            }}
            disabled={open && search != ""}
            placeholder="[Cmd+K/Ctrl+K] Type a command or search..."
          ></CommandInput>
        </Command>
        <Button variant="outline" onClick={() => setSearch("")}>
          <Cross1Icon className={cn("h-4", "w-4", "mr-2")} />
          Clear
        </Button>

        <ModeToggle />
      </div>

      {open && (
        <CommandDialog open={open} onOpenChange={setOpen}>
          <CommandInput
            value={search}
            onValueChange={setSearch}
            placeholder="Type a command or search..."
          ></CommandInput>
          <CommandEmpty>Nothing found...</CommandEmpty>
          <CommandList>
            <CommandGroup heading="Logs"></CommandGroup>
            <CommandGroup heading="Settings"></CommandGroup>
          </CommandList>
        </CommandDialog>
      )}

      <Separator></Separator>
    </>
  );
}
