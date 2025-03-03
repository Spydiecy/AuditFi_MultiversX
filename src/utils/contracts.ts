// ABI definition compatible with MultiversX SC format
export const AUDIT_REGISTRY_ABI = {
  endpoints: [
    {
      name: "register_audit",
      inputs: [
        { type: "bytes32", name: "contract_hash" },
        { type: "u8", name: "stars" },
        { type: "bytes", name: "summary" }
      ],
      outputs: []
    },
    {
      name: "get_all_audits",
      inputs: [
        { type: "u32", name: "start_index" },
        { type: "u32", name: "limit" }
      ],
      outputs: [
        { type: "multi<bytes32,u8,bytes,address,u64>" }
      ]
    },
    {
      name: "getTotalContracts",
      inputs: [],
      outputs: [{ type: "u32" }]
    },
    {
      name: "getContractAudits",
      inputs: [{ type: "bytes32", name: "contract_hash" }],
      outputs: [
        { 
          type: "variadic<multi<u8,bytes,address,u64>>",
          name: "audits"
        }
      ]
    },
    {
      name: "getAuditorHistory",
      inputs: [{ type: "address", name: "auditor" }],
      outputs: [{ type: "variadic<bytes32>" }]
    },
    {
      name: "getLatestAudit",
      inputs: [{ type: "bytes32", name: "contract_hash" }],
      outputs: [
        { type: "tuple<u8,bytes,address,u64>", name: "audit" }
      ]
    },
    {
      name: "withdraw",
      inputs: [],
      outputs: []
    },
    {
      name: "getOwner",
      inputs: [],
      outputs: [{ type: "address" }]
    }
  ],
  events: [
    {
      identifier: "auditRegistered",
      inputs: [
        { indexed: true, name: "contract_hash", type: "bytes32" },
        { indexed: true, name: "stars", type: "u8" },
        { indexed: true, name: "auditor", type: "address" },
        { name: "summary", type: "bytes" }
      ]
    }
  ]
};

// Contract addresses for each network
export const CONTRACT_ADDRESSES = {
  devnet: 'erd1qqqqqqqqqqqqqpgq0vng6wyq22ey3wstnyck8ed5k8z4yeqqd8ssf5fz9h',
  testnet: 'erd1qqqqqqqqqqqqqpgq0vng6wyq22ey3wstnyck8ed5k8z4yeqqd8ssf5fz9h',
  mainnet: ''
};

export type ChainKey = keyof typeof CONTRACT_ADDRESSES;

// Types for the Audit struct
export interface Audit {
  stars: number;
  summary: string;
  auditor: string;
  timestamp: number;
}

// Utility function to convert ASCII text to hex for MultiversX
export function stringToHex(text: string): string {
  return Array.from(text)
    .map(c => c.charCodeAt(0).toString(16).padStart(2, '0'))
    .join('');
}

// Utility function to convert hex to ASCII text
export function hexToString(hex: string): string {
  const hexString = hex.startsWith('0x') ? hex.slice(2) : hex;
  let str = '';
  for (let i = 0; i < hexString.length; i += 2) {
    str += String.fromCharCode(parseInt(hexString.substr(i, 2), 16));
  }
  return str;
}

// Utility function to generate a contract hash (from contract code)
export function generateContractHash(contractCode: string): string {
  // In a real implementation, you'd use keccak256 or another appropriate hash function
  // For simplicity, we'll just use a placeholder function here
  return '0x' + Array(64).fill(0).map(() => Math.floor(Math.random() * 16).toString(16)).join('');
}