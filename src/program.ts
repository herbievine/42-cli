import commander from "commander";
import figlet from "figlet";
import push from "./commands/push";

(async () => {
  console.log("\n");
  console.log(figlet.textSync("42  CLI", "Alligator"));
  console.log("\n");

  const program = new commander.Command();

  program
    .command("push")
    .description("Push a new project")
    .argument("<dir>", "path to project (~/42/libft)")
    .argument("<url>", "git url (https://github[...]libft.git)")
    .option(
      "-c, --clean <pattern>",
      "remove all files except the ones matching the regex"
    )
    .option("-n, --norm", "run norminette on your project")
    .action(push);

  await program.parseAsync(process.argv);
})();
