import { HeaderBarMenu } from "@/components/header-bar/menu";
import { ModeToggle } from "@/components/mode-toggle";
import { Button } from "@/components/ui/button";
import {
  Command,
  CommandDialog,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandList,
} from "@/components/ui/command";
import { Separator } from "@/components/ui/separator";
import { cn } from "@/lib/utils";
import { Cross1Icon } from "@radix-ui/react-icons";
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
    <>
      <div
        className={cn(
          "flex",
          "gap-1",
          "lg:max-w-4xl",
          "md:max-w-2xl",
          "mx-auto",
          "p-4"
        )}
      >
        <HeaderBarMenu />

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
          <Cross1Icon className={cn("h-4", "mr-2", "w-4")} />
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
