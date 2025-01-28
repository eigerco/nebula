export function findContractLineNumber(code: string) {
  const lines = code.split("\n");
  for (let index = 0; index < lines.length; index++) {
    const line = lines[index];
    if (line.trim().startsWith("#[contract]")) {
      return index + 1;
    }
  }
  return -1;
}

export function extractContractImplMethods(rustCode: string) {
  const lines = rustCode.split("\n");
  const methods: ContractFunction[] = [];
  let insideContractImpl = false;

  for (let index = 0; index < lines.length; index++) {
    const line = lines[index];
    const trimmedLine = line.trim();

    if (trimmedLine.startsWith("#[contractimpl]")) {
      insideContractImpl = true;
    } else if (
      insideContractImpl &&
      (trimmedLine.startsWith("pub fn") || trimmedLine.startsWith("fn"))
    ) {
      const methodSignature = trimmedLine.match(
        /fn (\w+)\(([^)]*)\) -> (\w+<*([^>]*)>*)\s*{/
      );
      if (methodSignature && methodSignature[1]) {
        const methodName = methodSignature[1];
        const parameters = methodSignature[2]
          .split(",")
          .map((param) => param.trim());
        const returnType = methodSignature[3];
        const returnTypeGenerics = methodSignature[4]
          .split(",")
          .map((type) => type.trim());

        methods.push({
          method: methodName,
          parameters: parameters.reduce(
            (prev, str) => {
              let [key, value] = str.split(":");
              return {
                ...prev,
                [key.trim()]: value.trim(),
              };
            },
            {} as Record<string, string>
          ),
          returnType: returnType,
          lineNumber: index + 1,
          returnTypeGenerics: returnTypeGenerics,
        });
      }
    } else if (insideContractImpl && line.startsWith("}")) {
      insideContractImpl = false;
      break;
    }
  }

  return methods;
}

export interface ContractFunction {
  method: string;
  parameters: Record<string, string>;
  returnType: string;
  lineNumber: number;
  returnTypeGenerics: string[];
}

export function findContractEvents(code: string): Subscription[] {
  const lines = code.split("\n");
  const events = [];
  for (let index = 0; index < lines.length; index++) {
    const line = lines[index];
    if (line.trim().startsWith("env.events()")) {
      let spacesAtStart = line.length - line.trimStart().length;
      events.push({
        lineNumber: index + 1,
        event: "🔔 Subscribe",
        spacesAtStart,
      });
    }
  }
  return events;
}

export interface Subscription {
  lineNumber: number;
  event: string;
  spacesAtStart: number;
}
