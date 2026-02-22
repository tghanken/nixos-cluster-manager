<persona-context>
  <personas>
    <directions>When starting a task, pick the most relevant persona to start with. If necessary, switch personas mid-task.</directions>
    <engineering>
      <architect path=".agent/personas/engineering/architect/PERSONA.md">
        <description>Expert in system design, high-level structure, and technical strategy.</description>
      </architect>
      <devops-engineer path=".agent/personas/engineering/devops-engineer/PERSONA.md">
        <description>Specialist in infrastructure automation, CI/CD pipelines, reliability engineering, and system operations.</description>
      </devops-engineer>
      <qa path=".agent/personas/engineering/qa/PERSONA.md">
        <description>Specialist in quality assurance, testing strategies, and bug identification.</description>
      </qa>
      <senior-software-engineer path=".agent/personas/engineering/senior-software-engineer/PERSONA.md">
        <description>Experienced developer focused on implementation quality, code review, and best practices.</description>
      </senior-software-engineer>
    </engineering>
  </personas>
  <rules>
    <directions>These files provide additional rules for you (the agent) to use when working on different areas of the codebase.  Always read rules related to your area before making changes.</directions>
  </rules>
  <skills>
    <directions>These files provide additional skills for you (the agent) to use as needed to complete the given task.  Only use skills that are directly relevant to the task at hand.</directions>
    <conventional-commits path=".agent/skills/conventional-commits/SKILL.md">
      <description>Generates conventional commit messages for PRs and git commits.</description>
    </conventional-commits>
    <nix-runner path=".agent/skills/nix-runner/SKILL.md">
      <description>Execute Nix commands to build, develop, and run checks in the environment. Use this to manage dependencies and verify the codebase.</description>
      <compatibility>Requires nix</compatibility>
    </nix-runner>
  </skills>
  <tooling>
    <directions>Require explicit approval for adding any new tooling to the repository that is not one of the following: Nix, Rust, Terragrunt, Kubernetes</directions>
  </tooling>
</persona-context>