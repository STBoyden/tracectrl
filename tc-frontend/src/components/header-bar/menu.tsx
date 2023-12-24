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
import { GearIcon } from "@radix-ui/react-icons";
import { useForm } from "react-hook-form";
import * as z from "zod";
import {
	_defaultSettings,
	settingsFormSchema,
	useSettings,
} from "@/components/settings-provider";

export function HeaderBarMenu() {
	const { toast } = useToast();
	const { settings, setSettings } = useSettings();

	const settingsForm = useForm<z.infer<typeof settingsFormSchema>>({
		resolver: zodResolver(settingsFormSchema),
		defaultValues: _defaultSettings,
	});

	async function onFormSubmit(values: z.infer<typeof settingsFormSchema>) {
		setSettings(values);

		toast({
			title: "Saved changes",
			description: (
				<>
					Host changed to <code>{values.websocketHost}</code>
				</>
			),
		});
	}

	return (
		<Sheet>
			<SheetTrigger asChild>
				<Button variant="outline">
					<GearIcon className={cn("h-4", "mr-2", "w-4")} /> Settings
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
							name="websocketHost"
							render={({ field }) => (
								<FormItem>
									<FormLabel>Host</FormLabel>
									<FormControl>
										<Input placeholder={settings.websocketHost!} {...field} />
									</FormControl>
									<FormDescription>
										The host from where to receive the logs via websockets from.
										By default, it is targeted at{" "}
										<span className={cn("font-mono")}>
											{_defaultSettings.websocketHost!}
										</span>
										.
									</FormDescription>
									<FormMessage />
								</FormItem>
							)}
						/>
						<Button type="submit">Save changes</Button>
					</form>
				</Form>
			</SheetContent>
		</Sheet>
	);
}
