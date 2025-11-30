import {z} from "zod";


export const V2flySchema = z.object({
    log: z.object({
        access: z.string().optional(),
        error: z.string().optional(),
        loglevel: z.enum(["debug", "info", "warning", "error", "none"]).optional(),
        dnsLog: z.boolean().optional(),
        maskAddress: z.enum(["quarter", "half", "full"]).optional()
    }).optional(),
    api: z.object({
        tag: z.string().optional(),
        listen: z.string().optional(),
        services: z.array(z.string()).optional()
    }).optional(),
    dns: z.object({
        hosts: z.record(z.string(), z.union([
            z.string(),
            z.array(z.string())
        ])).optional(),
        servers: z.array(z.union([
            z.string(),
            z.object({
                address: z.string(),
                port: z.number().optional(),
                domains: z.array(z.string()).optional(),
                expectedIPs: z.array(z.string()).optional(),
                unexpectedIPs: z.array(z.string()).optional(),
                skipFallback: z.boolean().optional(),
                clientIP: z.string().optional(),
                queryStrategy: z.enum(["UseIP", "UseIPv4", "UseIPv6", "UseSystem"]).optional(),
                tag: z.string().optional(),
                timeoutMs: z.number().optional(),
                disableCache: z.boolean().optional(),
                finalQuery: z.boolean().optional()
            })
        ])).optional(),
        clientIp: z.string().optional(),
        queryStrategy: z.enum(["UseIP", "UseIPv4", "UseIPv6", "UseSystem"]).optional(),
        disableCache: z.boolean().optional(),
        disableFallback: z.boolean().optional(),
        disableFallbackIfMatch: z.boolean().optional(),
        useSystemHosts: z.boolean().optional(),
        tag: z.string().optional()
    }).optional(),
    routing: z.object({
        domainStrategy: z.enum(["AsIs", "IPIfNonMatch", "IPOnDemand"]).optional(),
        domainMatcher: z.enum(["hybrid", "linear"]).optional(),
        rules: z.array(z.object({
            domainMatcher: z.enum(["hybrid", "linear"]).optional(),
            type: z.literal("field").optional(),
            domain: z.array(z.string()).optional(),
            ip: z.array(z.string()).optional(),
            port: z.union([z.number(), z.string()]).optional(),
            sourcePort: z.union([z.number(), z.string()]).optional(),
            network: z.enum(["tcp", "udp", "tcp,udp"]).optional(),
            source: z.array(z.string()).optional(),
            user: z.array(z.string()).optional(),
            inboundTag: z.array(z.string()).optional(),
            protocol: z.array(z.enum(["http", "tls", "bittorrent"])).optional(),
            attrs: z.record(z.string(), z.string()).optional(),
            outboundTag: z.string().optional(),
            balancerTag: z.string().optional()
        })).optional(),
        balancers: z.array(z.object({
            tag: z.string().optional(),
            selector: z.array(z.string()).optional()
        })).optional()
    }).optional(),
    policy: z.object({
        levels: z.record(z.string(), z.object({
            handshake: z.number().optional(),
            connIdle: z.number().optional(),
            uplinkOnly: z.number().optional(),
            downlinkOnly: z.number().optional(),
            statsUserUplink: z.boolean().optional(),
            statsUserDownlink: z.boolean().optional(),
            statsUserOnline: z.boolean().optional(),
            bufferSize: z.number().optional()
        })).optional(),
        system: z.object({
            statsInboundUplink: z.boolean().optional(),
            statsInboundDownlink: z.boolean().optional(),
            statsOutboundUplink: z.boolean().optional(),
            statsOutboundDownlink: z.boolean().optional()
        }).optional()
    }).optional(),
    inbounds: z.array(z.object({
        listen: z.string().optional(),
        port: z.union([z.number(), z.string()]).optional(),
        protocol: z.enum(["dokodemo-door", "http", "shadowsocks", "socks", "vless", "vmess", "trojan", "wireguard"]).optional(),
        settings: z.record(z.string(), z.any()).optional(),
        streamSettings: z.record(z.string(), z.any()).optional(),
        tag: z.string().optional(),
        sniffing: z.object({
            enabled: z.boolean().optional(),
            destOverride: z.array(z.enum(["http", "tls", "quic", "fakedns"])).optional(),
            metadataOnly: z.boolean().optional(),
            domainsExcluded: z.array(z.string()).optional(),
            routeOnly: z.boolean().optional()
        }).optional(),
        allocate: z.object({
            strategy: z.enum(["always", "random"]).optional(),
            refresh: z.number().optional(),
            concurrency: z.number().optional()
        }).optional()
    })).optional(),
    outbounds: z.array(z.object({
        sendThrough: z.string().optional(),
        protocol: z.string().optional(),
        settings: z.record(z.string(), z.any()).optional(),
        tag: z.string().optional(),
        streamSettings: z.record(z.string(), z.any()).optional(),
        proxySettings: z.object({
            tag: z.string().optional()
        }).optional(),
        mux: z.object({
            enabled: z.boolean().optional(),
            concurrency: z.number().optional(),
            xudpConcurrency: z.number().optional(),
            xudpProxyUDP443: z.enum(["reject", "allow", "skip"]).optional()
        }).optional(),
        targetStrategy: z.enum([
            "AsIs", "UseIP", "UseIPv6v4", "UseIPv6", "UseIPv4v6", "UseIPv4",
            "ForceIP", "ForceIPv6v4", "ForceIPv6", "ForceIPv4v6", "ForceIPv4"
        ]).optional()
    })).optional(),
    transport: z.object({
        network: z.enum(["raw", "xhttp", "kcp", "grpc", "ws", "httpupgrade"]).optional(),
        security: z.enum(["none", "tls", "reality"]).optional(),
        tlsSettings: z.object({
            serverName: z.string().optional(),
            rejectUnknownSni: z.boolean().optional(),
            verifyPeerCertInNames: z.array(z.string()).optional(),
            allowInsecure: z.boolean().optional(),
            alpn: z.array(z.string()).optional(),
            minVersion: z.string().optional(),
            maxVersion: z.string().optional(),
            cipherSuites: z.string().optional(),
            certificates: z.array(z.object({
                ocspStapling: z.number().optional(),
                oneTimeLoading: z.boolean().optional(),
                usage: z.enum(["encipherment", "verify", "issue"]).optional(),
                buildChain: z.boolean().optional(),
                certificateFile: z.string().optional(),
                keyFile: z.string().optional(),
                certificate: z.array(z.string()).optional()
            })).optional(),
            disableSystemRoot: z.boolean().optional(),
            enableSessionResumption: z.boolean().optional(),
            fingerprint: z.string().optional(),
            pinnedPeerCertificateChainSha256: z.array(z.string()).optional(),
            masterKeyLog: z.string().optional()
        }).optional(),
        realitySettings: z.object({
            show: z.boolean().optional(),
            dest: z.string().optional(),
            xver: z.number().optional(),
            serverNames: z.array(z.string()).optional(),
            privateKey: z.string().optional(),
            minClientVer: z.string().optional(),
            maxClientVer: z.string().optional(),
            maxTimeDiff: z.number().optional(),
            shortIds: z.array(z.string()).optional(),
            fingerprint: z.string().optional(),
            serverName: z.string().optional(),
            publicKey: z.string().optional(),
            shortId: z.string().optional(),
            spiderX: z.string().optional()
        }).optional(),
        rawSettings: z.object({}).optional(),
        xhttpSettings: z.object({}).optional(),
        kcpSettings: z.object({}).optional(),
        grpcSettings: z.object({}).optional(),
        wsSettings: z.object({}).optional(),
        httpupgradeSettings: z.object({}).optional(),
        sockopt: z.object({
            mark: z.number().optional(),
            tcpMaxSeg: z.number().optional(),
            tcpFastOpen: z.boolean().optional(),
            tproxy: z.enum(["off", "redirect", "tproxy"]).optional(),
            domainStrategy: z.enum(["AsIs", "UseIP", "UseIPv4", "UseIPv6"]).optional(),
            happyEyeballs: z.object({
                tryDelayMs: z.number().optional()
            }).optional(),
            dialerProxy: z.string().optional(),
            acceptProxyProtocol: z.boolean().optional(),
            tcpKeepAliveInterval: z.number().optional(),
            tcpKeepAliveIdle: z.number().optional(),
            tcpUserTimeout: z.number().optional(),
            tcpCongestion: z.string().optional(),
            interface: z.string().optional(),
            v6only: z.boolean().optional(),
            tcpWindowClamp: z.number().optional(),
            tcpMptcp: z.boolean().optional(),
            tcpNoDelay: z.boolean().optional()
        }).optional()
    }).optional(),
    stats: z.object({
        // StatsObject currently doesn't require any parameters
        // Internal statistics will be enabled as long as this object exists
    }).optional(),
    reverse: z.object({
        bridges: z.array(z.object({
            tag: z.string().optional(),
            domain: z.string().optional()
        })).optional(),
        portals: z.array(z.object({
            tag: z.string().optional(),
            domain: z.string().optional()
        })).optional()
    }).optional(),
    fakedns: z.union([
        z.object({
            ipPool: z.string().optional(),
            poolSize: z.number().optional()
        }).optional(),
        z.array(z.object({
            ipPool: z.string(),
            poolSize: z.number().optional()
        }))
    ]).optional(),
    metrics: z.object({
        tag: z.string().optional()
    }).optional(),
    observatory: z.object({
        subjectSelector: z.array(z.string()).optional(),
        probeUrl: z.string().optional(),
        probeInterval: z.string().optional(),
        enableConcurrency: z.boolean().optional()
    }).optional(),
    burstObservatory: z.object({
        subjectSelector: z.array(z.string()).optional(),
        pingConfig: z.object({
            destination: z.string().optional(),
            connectivity: z.string().optional(),
            interval: z.string().optional(),
            sampling: z.number().optional(),
            timeout: z.string().optional()
        }).optional()
    }).optional()
})

export type v2flyConfig = z.infer<typeof V2flySchema>;
