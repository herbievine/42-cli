#!/usr/bin/env ts-node

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
    .argument("<project_directory>", "path to project (~/42/libft)")
    .argument("<git_repository>", "git url (`https` or `ssh`)")
    .option(
      "-i, --include <pattern>",
      "includes only the files matching the pattern",
      ""
    )
    .option("-n, --norm", "run norminette on your project", false)
    .action(push);

  await program.parseAsync(process.argv);
})();
