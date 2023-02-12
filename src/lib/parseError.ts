import { red, bold, underline } from "colorette";

const parseError = (message: string, stack: unknown, exit?: boolean) => {
  console.error("\n" + red(message));

  if (stack instanceof Error) {
    console.error("\n" + red(stack.message));
  }

  if (exit) {
    process.exit(1);
  }
};

export default parseError;
