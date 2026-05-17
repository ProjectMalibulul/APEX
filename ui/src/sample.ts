import type { GraphDocument } from "./lib/types.js";

export const sampleGraph: GraphDocument = {
  nodes: [
    { id: "type:UserService", name: "UserService", kind: "type", path: "ts/UserService.ts", layer: "service" },
    { id: "type:UserRepository", name: "UserRepository", kind: "type", path: "ts/UserRepository.ts", layer: "data" },
    { id: "type:TokenValidator", name: "TokenValidator", kind: "type", path: "ts/api/TokenValidator.ts", layer: "api" },
    { id: "type:CacheStore", name: "CacheStore", kind: "type", path: "ts/infrastructure/CacheStore.ts", layer: "infrastructure" }
  ],
  edges: [
    { from: "type:UserService", to: "type:UserRepository", kind: "imports" },
    { from: "type:UserService", to: "type:TokenValidator", kind: "imports" },
    { from: "type:TokenValidator", to: "type:CacheStore", kind: "imports" }
  ]
};

