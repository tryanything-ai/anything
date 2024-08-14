import { Button } from "@repo/ui/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@repo/ui/components/ui/dialog";
import { ShareIcon } from "lucide-react";
import Link from "next/link";

export function ShareDialog(): JSX.Element {
  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button variant="outline" size="sm" className="ml-auto gap-1.5 text-sm">
          <ShareIcon className="size-3.5" />
          Share
        </Button>
      </DialogTrigger>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>Share workflow as template</DialogTitle>
          <DialogDescription>Get credit for your creativity</DialogDescription>
        </DialogHeader>
        <div className="flex flex-col space-x-2">
          <div>
            Other users will be able to install and use your template in their
            business from the{" "}
            <Link
              href="https://tryanything.xyz/templates"
              target="_blank"
              rel="noopener noreferrer"
              className="text-blue-500 underline"
            >
              Anything Marketplace
            </Link>
          </div>

          {/* <div className="grid flex-1 gap-2">
            <Label htmlFor="link" className="sr-only">
              Link
            </Label>
            <Input
              id="link"
              defaultValue="https://ui.shadcn.com/docs/installation"
              readOnly
            />
          </div>
          <Button type="submit" size="sm" className="px-3">
            <span className="sr-only">Copy</span>
            <Copy className="h-4 w-4" />
          </Button> */}
        </div>
        <DialogFooter className="sm:justify-start">
          {/* <DialogClose asChild> */}
          <Button className="mt-2">Publish To Anything Marketplace</Button>
          {/* </DialogClose> */}
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
