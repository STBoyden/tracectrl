import { Button } from "@/components/ui/button";
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import {
  Sheet,
  SheetContent,
  SheetDescription,
  SheetHeader,
  SheetTitle,
  SheetTrigger,
} from "@/components/ui/sheet";
import { useToast } from "@/components/ui/use-toast";
import { cn } from "@/lib/utils";
import { zodResolver } from "@hookform/resolvers/zod";
import { HamburgerMenuIcon } from "@radix-ui/react-icons";
import { useForm } from "react-hook-form";
import * as z from "zod";
import { _defaultSettings, settingsFormSchema } from "../settings-provider";

export function HeaderBarMenu() {
  const { toast } = useToast();

  const settingsForm = useForm<z.infer<typeof settingsFormSchema>>({
    resolver: zodResolver(settingsFormSchema),
    defaultValues: _defaultSettings,
  });

  async function onFormSubmit(values: z.infer<typeof settingsFormSchema>) {
    toast({
      title: "Saved changes",
    });
  }

  return (
    <Sheet>
      <SheetTrigger asChild>
        <Button variant="outline">
          <HamburgerMenuIcon className={cn("h-4", "mr-2", "w-4")} /> Menu
        </Button>
      </SheetTrigger>
      <SheetContent side="left">
        <SheetHeader>
          <SheetTitle>Settings</SheetTitle>
          <SheetDescription>
            Settings for TraceCTRL can be found here.
          </SheetDescription>
        </SheetHeader>
        <Form {...settingsForm}>
          <form
            onSubmit={settingsForm.handleSubmit(onFormSubmit)}
            className={cn("space-y-4")}
          >
            <FormField
              control={settingsForm.control}
              name="host"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Host</FormLabel>
                  <FormControl>
                    <Input placeholder={_defaultSettings.host!} {...field} />
                  </FormControl>
                  <FormDescription>
                    The host from where to receive the logs from. By default, it
                    is targeted at{" "}
                    <span className={cn("font-mono")}>
                      {_defaultSettings.host!}
                    </span>
                    .
                  </FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            ></FormField>
            <Button type="submit">Save changes</Button>
          </form>
        </Form>
      </SheetContent>
    </Sheet>
  );
}
