use std::str::FromStr;
use std::fmt;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::num::ParseIntError;

enum SdpParserResult {
    ParserLineError   { message: String,
                        line: String },
    ParserUnsupported { message: String,
                        line: String },
}

impl From<ParseIntError> for SdpParserResult {
    fn from(_: ParseIntError) -> SdpParserResult {
        // TODO empty line error here makes no sense
        SdpParserResult::ParserLineError { message: "failed to parse integer".to_string(),
                                           line: "".to_string() }
    }
}

enum SdpAttributeType {
    // TODO consolidate these into groups
    Candidate,
    EndOfCandidates,
    Extmap,
    Fingerprint,
    Fmtp,
    Group,
    IceOptions,
    IcePwd,
    IceUfrag,
    Inactive,
    Mid,
    Msid,
    MsidSemantic,
    Rid,
    Recvonly,
    Rtcp,
    RtcpFb,
    RtcpMux,
    RtcpRsize,
    Rtpmap,
    Sctpmap,
    SctpPort,
    Sendonly,
    Sendrecv,
    Setup,
    Simulcast,
    Ssrc,
    SsrcGroup,
}

impl fmt::Display for SdpAttributeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            SdpAttributeType::Candidate => "Candidate",
            SdpAttributeType::EndOfCandidates => "End-Of-Candidates",
            SdpAttributeType::Extmap => "Extmap",
            SdpAttributeType::Fingerprint => "Fingerprint",
            SdpAttributeType::Fmtp => "Fmtp",
            SdpAttributeType::Group => "Group",
            SdpAttributeType::IceOptions => "Ice-Options",
            SdpAttributeType::IcePwd => "Ice-Pwd",
            SdpAttributeType::IceUfrag => "Ice-Ufrag",
            SdpAttributeType::Inactive => "Inactive",
            SdpAttributeType::Mid => "Mid",
            SdpAttributeType::Msid => "Msid",
            SdpAttributeType::MsidSemantic => "Msid-Semantic",
            SdpAttributeType::Rid => "Rid",
            SdpAttributeType::Recvonly => "Recvonly",
            SdpAttributeType::Rtcp => "Rtcp",
            SdpAttributeType::RtcpFb => "Rtcp-Fb",
            SdpAttributeType::RtcpMux => "Rtcp-Mux",
            SdpAttributeType::RtcpRsize => "Rtcp-Rsize",
            SdpAttributeType::Rtpmap => "Rtpmap",
            SdpAttributeType::Sctpmap => "Sctpmap",
            SdpAttributeType::SctpPort => "Sctp-Port",
            SdpAttributeType::Sendonly => "Sendonly",
            SdpAttributeType::Sendrecv => "Sendrecv",
            SdpAttributeType::Setup => "Setup",
            SdpAttributeType::Simulcast => "Simulcast",
            SdpAttributeType::Ssrc => "Ssrc",
            SdpAttributeType::SsrcGroup => "Ssrc-Group",
        };
        write!(f, "{}", printable)
    }
}

struct SdpAttribute {
    name: SdpAttributeType,
    value: String
}

struct SdpBandwidth {
    bwtype: String,
    bandwidth: u64
}

enum SdpNetType {
    Internet
}

impl fmt::Display for SdpNetType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IN")
    }
}

enum SdpAddrType {
    IP4,
    IP6
}

impl fmt::Display for SdpAddrType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            SdpAddrType::IP4 => "Ip4",
            SdpAddrType::IP6 => "Ip6"
        };
        write!(f, "{}", printable)
    }
}

struct SdpConnection {
    nettype: SdpNetType,
    addrtype: SdpAddrType,
    unicast_addr: IpAddr
}

enum SdpMediaValue {
    Audio,
    Video,
    Application
}

impl fmt::Display for SdpMediaValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            SdpMediaValue::Audio       => "Audio",
            SdpMediaValue::Video       => "Video",
            SdpMediaValue::Application => "Application"
        };
        write!(f, "{}", printable)
    }
}

enum SdpProtocolValue {
    UdpTlsRtpSavpf,
    TcpTlsRtpSavpf,
    DtlsSctp,
    UdpDtlsSctp,
    TcpDtlsSctp
}

impl fmt::Display for SdpProtocolValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            SdpProtocolValue::UdpTlsRtpSavpf => "Udp/Tls/Rtp/Savpf",
            SdpProtocolValue::TcpTlsRtpSavpf => "Tcp/Tls/Rtp/Savpf",
            SdpProtocolValue::DtlsSctp       => "Dtls/Sctp",
            SdpProtocolValue::UdpDtlsSctp    => "Udp/Dtls/Sctp",
            SdpProtocolValue::TcpDtlsSctp    => "Tcp/Dtls/Sctp"
        };
        write!(f, "{}", printable)
    }
}

enum SdpFormatList {
    Integers {list: Vec<u32>},
    Strings {list: Vec<String>}
}

impl fmt::Display for SdpFormatList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SdpFormatList::Integers { list: ref x } => write!(f, "{:?}", x),
            SdpFormatList::Strings { list: ref x } => write!(f, "{:?}", x)
        }
    }
}

struct SdpMediaLine {
    media: SdpMediaValue,
    port: u32,
    proto: SdpProtocolValue,
    formats: SdpFormatList
}

struct SdpOrigin {
    username: String,
    session_id: u64,
    session_version: u64,
    nettype: SdpNetType,
    addrtype: SdpAddrType,
    unicast_addr: IpAddr
}

struct SdpTiming {
    start: u64,
    stop: u64
}

enum SdpLine {
    Attribute {value: SdpAttribute},
    Bandwidth {value: SdpBandwidth},
    Connection {value: SdpConnection},
    Email {value: String},
    Information {value: String},
    Key {value: String},
    Media {value: SdpMediaLine},
    Phone {value: String},
    Origin {value: SdpOrigin},
    Repeat {value: String},
    Session {value: String},
    Timing {value: SdpTiming},
    Uri {value: String},
    Version {value: u64},
    Zone {value: String}
}

struct SdpMedia {
    media: SdpLine,
    information: Option<String>,
    connection: SdpConnection,
    bandwidth: Option<SdpBandwidth>,
    key: Option<String>,
    attribute: Option<SdpAttribute>
}

struct SdpSession {
    version: u64,
    origin: SdpOrigin,
    session: String,
    information: Option<String>,
    uri: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    connection: Option<SdpConnection>,
    bandwidth: Option<SdpBandwidth>,
    timing: SdpTiming,
    key: Option<String>,
    attribute: Option<SdpAttribute>,
    media: SdpMedia
}

fn parse_repeat(value: &str) -> Result<SdpLine, SdpParserResult> {
    // TODO implement this if it's ever needed
    println!("repeat: {}", value);
    Result::Ok(SdpLine::Repeat{value: String::from(value)})
}

fn parse_zone(value: &str) -> Result<SdpLine, SdpParserResult> {
    // TODO implement this if it's ever needed
    println!("zone: {}", value);
    Result::Ok(SdpLine::Zone {value: String::from(value)})
}

fn parse_key(value: &str) -> Result<SdpLine, SdpParserResult> {
    // TODO implement this if it's ever needed
    println!("key: {}", value);
    Result::Ok(SdpLine::Key {value: String::from(value)})
}

fn parse_information(value: &str) -> Result<SdpLine, SdpParserResult> {
    println!("information: {}", value);
    Result::Ok(SdpLine::Information {value: String::from(value)})
}

fn parse_uri(value: &str) -> Result<SdpLine, SdpParserResult> {
    // TODO check if this is really a URI
    println!("uri: {}", value);
    Result::Ok(SdpLine::Uri {value: String::from(value)})
}

fn parse_email(value: &str) -> Result<SdpLine, SdpParserResult> {
    // TODO check if this is really an email address
    println!("email: {}", value);
    Result::Ok(SdpLine::Email {value: String::from(value)})
}

fn parse_phone(value: &str) -> Result<SdpLine, SdpParserResult> {
    // TODO check if this is really a phone number
    println!("phone: {}", value);
    Result::Ok(SdpLine::Phone {value: String::from(value)})
}

fn parse_session(value: &str) -> Result<SdpLine, SdpParserResult> {
    println!("session: {}", value);
    Result::Ok(SdpLine::Session {value: String::from(value)})
}

fn parse_version(value: &str) -> Result<SdpLine, SdpParserResult> {
    let ver = try!(value.parse::<u64>());
    if ver != 0 {
        return Result::Err(SdpParserResult::ParserLineError {
            message: "unsupported version in v field".to_string(),
            line: value.to_string() });
    };
    println!("version: {}", ver);
    Result::Ok(SdpLine::Version { value: ver })
}

fn parse_nettype(value: &str) -> Result<SdpNetType, SdpParserResult> {
    if value.to_uppercase() != String::from("IN") {
        return Result::Err(SdpParserResult::ParserLineError {
            message: "nettype needs to be IN".to_string(),
            line: value.to_string() });
    };
    Result::Ok(SdpNetType::Internet)
}

fn parse_addrtype(value: &str) -> Result<SdpAddrType, SdpParserResult> {
    Result::Ok(match value.to_uppercase().as_ref() {
        "IP4" => SdpAddrType::IP4,
        "IP6" => SdpAddrType::IP6,
        _ => return Result::Err(SdpParserResult::ParserLineError {
            message: "address type needs to be IP4 or IP6".to_string(),
            line: value.to_string() })
    })
}

fn parse_unicast_addr(addrtype: &SdpAddrType, value: &str) -> Result<IpAddr, SdpParserResult> {
    Result::Ok(match addrtype {
        &SdpAddrType::IP4 => {
            IpAddr::V4(match Ipv4Addr::from_str(value) {
                Ok(n) => n,
                Err(_) => return Result::Err(SdpParserResult::ParserLineError {
                    message: "failed to parse unicast IP4 address attribute".to_string(),
                    line: value.to_string() })
            })
        },
        &SdpAddrType::IP6 => {
            IpAddr::V6(match Ipv6Addr::from_str(value) {
                Ok(n) => n,
                Err(_) => return Result::Err(SdpParserResult::ParserLineError {
                    message: "failed to parse unicast IP6 address attribute".to_string(),
                    line: value.to_string() })
            })
        }
    })
}

fn parse_origin(value: &str) -> Result<SdpLine, SdpParserResult> {
    let ot: Vec<&str> = value.split_whitespace().collect();
    if ot.len() != 6 {
        return Result::Err(SdpParserResult::ParserLineError {
            message: "origin field must have six tokens".to_string(),
            line: value.to_string() });
    }
    let username = ot[0];
    let session_id = try!(ot[1].parse::<u64>());
    let session_version = try!(ot[2].parse::<u64>());
    let nettype = try!(parse_nettype(ot[3]));
    let addrtype = try!(parse_addrtype(ot[4]));
    let unicast_addr = try!(parse_unicast_addr(&addrtype, ot[5]));
    let o = SdpOrigin { username: String::from(username),
                        session_id: session_id,
                        session_version: session_version,
                        nettype: nettype,
                        addrtype: addrtype,
                        unicast_addr: unicast_addr };
    println!("origin: {}, {}, {}, {}, {}, {}",
             o.username, o.session_id, o.session_version, o.nettype,
             o.addrtype, o.unicast_addr);
    Result::Ok(SdpLine::Origin { value: o })
}

fn parse_connection(value: &str) -> Result<SdpLine, SdpParserResult> {
    let cv: Vec<&str> = value.split_whitespace().collect();
    if cv.len() != 3 {
        return Result::Err(SdpParserResult::ParserLineError {
            message: "connection attribute must have three tokens".to_string(),
            line: value.to_string() });
    }
    // TODO this is exactly the same parser as the end of origin.
    //      Share it in a function?!
    let nettype = try!(parse_nettype(cv[0]));
    let addrtype = try!(parse_addrtype(cv[1]));
    let unicast_addr = try!(parse_unicast_addr(&addrtype, cv[2]));
    let c = SdpConnection { nettype: nettype,
                            addrtype: addrtype,
                            unicast_addr: unicast_addr };
    println!("connection: {}, {}, {}",
             c.nettype, c.addrtype, c.unicast_addr);
    Result::Ok(SdpLine::Connection { value: c })
}

fn parse_bandwidth(value: &str) -> Result<SdpLine, SdpParserResult> {
    let bv: Vec<&str> = value.split(':').collect();
    if bv.len() != 2 {
        return Result::Err(SdpParserResult::ParserLineError {
            message: "bandwidth attribute must have two tokens".to_string(),
            line: value.to_string() });
    }
    let bwtype = bv[0];
    match bwtype.to_uppercase().as_ref() {
        "AS" | "TIAS" => (),
        _ => return Result::Err(SdpParserResult::ParserUnsupported {
              message: "unsupported bandwidth type value".to_string(),
              line: value.to_string() }),
    };
    let bandwidth = try!(bv[1].parse::<u64>());
    let b = SdpBandwidth { bwtype: String::from(bwtype),
                            bandwidth: bandwidth };
    println!("bandwidth: {}, {}",
             b.bwtype, b.bandwidth);
    Result::Ok(SdpLine::Bandwidth { value: b })
}

fn parse_timing(value: &str) -> Result<SdpLine, SdpParserResult> {
    let tv: Vec<&str> = value.split_whitespace().collect();
    if tv.len() != 2 {
        return Result::Err(SdpParserResult::ParserLineError {
            message: "timing attribute must have two tokens".to_string(),
            line: value.to_string() });
    }
    let start_time = try!(tv[0].parse::<u64>());
    let stop_time = try!(tv[1].parse::<u64>());
    let t = SdpTiming { start: start_time,
                        stop: stop_time };
    println!("timing: {}, {}", t.start, t.stop);
    Result::Ok(SdpLine::Timing { value: t })
}

fn parse_media_token(value: &str) -> Result<SdpMediaValue, SdpParserResult> {
    Result::Ok(match value.to_lowercase().as_ref() {
        "audio"       => SdpMediaValue::Audio,
        "video"       => SdpMediaValue::Video,
        "application" => SdpMediaValue::Application,
        _ => return Result::Err(SdpParserResult::ParserUnsupported {
              message: "unsupported media value".to_string(),
              line: value.to_string() }),
    })
}

fn parse_protocol_token(value: &str) -> Result<SdpProtocolValue, SdpParserResult> {
    Result::Ok(match value.to_uppercase().as_ref() {
        "UDP/TLS/RTP/SAVPF" => SdpProtocolValue::UdpTlsRtpSavpf,
        "TCP/TLS/RTP/SAVPF" => SdpProtocolValue::TcpTlsRtpSavpf,
        "DTLS/SCTP"         => SdpProtocolValue::DtlsSctp,
        "UDP/DTLS/SCTP"     => SdpProtocolValue::UdpDtlsSctp,
        "TCP/DTLS/SCTP"     => SdpProtocolValue::TcpDtlsSctp,
        _ => return Result::Err(SdpParserResult::ParserUnsupported {
              message: "unsupported protocol value".to_string(),
              line: value.to_string() }),
    })
}

fn parse_media(value: &str) -> Result<SdpLine, SdpParserResult> {
    let mv: Vec<&str> = value.split_whitespace().collect();
    if mv.len() < 4 {
        return Result::Err(SdpParserResult::ParserLineError {
            message: "media attribute must have at least four tokens".to_string(),
            line: value.to_string() });
    }
    let media = try!(parse_media_token(mv[0]));
    let port = try!(mv[1].parse::<u32>());
    if port > 65535 {
        return Result::Err(SdpParserResult::ParserLineError {
            message: "media port token is too big".to_string(),
            line: value.to_string() })
    }
    let proto = try!(parse_protocol_token(mv[2]));
    let fmt_slice: &[&str] = &mv[3..];
    let fmt = match media {
        SdpMediaValue::Audio | SdpMediaValue::Video => {
            let mut fmt_vec: Vec<u32> = vec![];
            for num in fmt_slice {
                let fmt_num = try!(num.parse::<u32>());
                match fmt_num {
                    0 => (),           // PCMU
                    8 => (),           // PCMA
                    9 => (),           // G722
                    13 => (),          // Comfort Noise
                    96 ... 127 => (),  // dynamic range
                    _ => return Result::Err(SdpParserResult::ParserLineError {
                          message: "format number in media line is out of range".to_string(),
                          line: value.to_string() }),
                };
                fmt_vec.push(fmt_num);
            };
            SdpFormatList::Integers { list: fmt_vec }
        },
        SdpMediaValue::Application => {
            let mut fmt_vec: Vec<String> = vec![];
            // TODO enforce length == 1 and content 'webrtc-datachannel' only?
            for token in fmt_slice {
                fmt_vec.push(String::from(*token));
            }
            SdpFormatList::Strings { list: fmt_vec }
        }
    };
    let m = SdpMediaLine { media: media,
                           port: port,
                           proto: proto,
                           formats: fmt };
    println!("media: {}, {}, {}, {}",
             m.media, m.port, m.proto, m.formats);
    Result::Ok(SdpLine::Media { value: m })
}

fn parse_attribute(value: &str) -> Result<SdpLine, SdpParserResult> {
    let attribute = value;
    let colon = attribute.find(':');
    let name: &str;
    let mut value: &str = "";
    if colon == None {
        name = attribute;
    } else {
        let (aname, avalue) = attribute.split_at(colon.unwrap());
        name = aname;
        value = avalue;
    }
    let mut attrtype;
    match name.to_lowercase().as_ref() {
        // TODO TODO TODO
        "candidate" => { attrtype = SdpAttributeType::Candidate; },
        "end-of-candidates" => { attrtype = SdpAttributeType::EndOfCandidates; },
        "extmap" => { attrtype = SdpAttributeType::Extmap; },
        "fingerprint" => { attrtype = SdpAttributeType::Fingerprint; },
        "fmtp" => { attrtype = SdpAttributeType::Fmtp; },
        "group" => { attrtype = SdpAttributeType::Group; },
        "ice-options" => { attrtype = SdpAttributeType::IceOptions; },
        "ice-pwd" => { attrtype = SdpAttributeType::IcePwd; },
        "ice-ufrag" => { attrtype = SdpAttributeType::IceUfrag; },
        "inactive" => { attrtype = SdpAttributeType::Inactive; },
        "mid" => { attrtype = SdpAttributeType::Mid; },
        "msid" => { attrtype = SdpAttributeType::Msid; },
        "msid-semantic" => { attrtype = SdpAttributeType::MsidSemantic; },
        "rid" => { attrtype = SdpAttributeType::Rid; },
        "recvonly" => { attrtype = SdpAttributeType::Recvonly; },
        "rtcp" => { attrtype = SdpAttributeType::Rtcp; },
        "rtcp-fb" => { attrtype = SdpAttributeType::RtcpFb; },
        "rtcp-mux" => { attrtype = SdpAttributeType::RtcpMux; },
        "rtcp-rsize" => { attrtype = SdpAttributeType::RtcpRsize; },
        "rtpmap" => { attrtype = SdpAttributeType::Rtpmap; },
        "sctpmap" => { attrtype = SdpAttributeType::Sctpmap; },
        "sctp-port" => { attrtype = SdpAttributeType::SctpPort; },
        "sendonly" => { attrtype = SdpAttributeType::Sendonly; },
        "sendrecv" => { attrtype = SdpAttributeType::Sendrecv; },
        "setup" => { attrtype = SdpAttributeType::Setup; },
        "simulcast" => { attrtype = SdpAttributeType::Simulcast; },
        "ssrc" => { attrtype = SdpAttributeType::Ssrc; },
        "ssrc-group" => { attrtype = SdpAttributeType::SsrcGroup; },
        _ => return Result::Err(SdpParserResult::ParserUnsupported {
              message: "unsupported attribute value".to_string(),
              line: name.to_string() }),
    }
    let a = SdpAttribute { name: attrtype,
                           value: String::from(value) };
    println!("attribute: {}, {}", 
             a.name, a.value);
    Result::Ok(SdpLine::Attribute { value: a })
}

fn parse_sdp_line(line: &str) -> Result<SdpLine, SdpParserResult> {
    let v: Vec<&str> = line.splitn(2, '=').collect();
    if v.len() < 2 {
        return Result::Err(SdpParserResult::ParserLineError {
            message: "failed to split field and attribute".to_string(),
            line: line.to_string() });
    };
    let name = v[0].trim();
    if name.is_empty() || name.len() > 1 {
        return Result::Err(SdpParserResult::ParserLineError {
            message: "field name empty or too long".to_string(),
            line: line.to_string() });
    };
    let value = v[1].trim();
    if value.len() == 0 {
        return Result::Err(SdpParserResult::ParserLineError {
            message: "attribute value has zero length".to_string(),
            line: line.to_string() });
    }
    match name.to_lowercase().as_ref() {
        "a" => { parse_attribute(value) },
        "b" => { parse_bandwidth(value) },
        "c" => { parse_connection(value) },
        "e" => { parse_email(value) },
        "i" => { parse_information(value) },
        "k" => { parse_key(value) },
        "m" => { parse_media(value) },
        "o" => { parse_origin(value) },
        "p" => { parse_phone(value) },
        "r" => { parse_repeat(value) },
        "s" => { parse_session(value) },
        "t" => { parse_timing(value) },
        "u" => { parse_uri(value) },
        "v" => { parse_version(value) },
        "z" => { parse_zone(value) },
        _   => { return Result::Err(SdpParserResult::ParserLineError {
                    message: "unsupported sdp field".to_string(),
                    line: line.to_string() }) }
    }
}

fn parse_sdp_vector(lines: Vec<SdpLine>) -> Result<SdpSession, SdpParserResult> {
    if lines.len() < 5 {
        return Result::Err(SdpParserResult::ParserLineError {
            message: "SDP neeeds at least 5 lines".to_string(),
            line: "".to_string() })
    }
    /*
    version: u64,
    origin: SdpOrigin,
    session: String,
    information: Option<String>,
    uri: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    connection: Option<SdpConnection>,
    bandwidth: Option<SdpBandwidth>,
    timing: SdpTiming,
    key: Option<String>,
    attribute: Option<SdpAttribute>,
    media: SdpMedia
    */
    match lines[0] {
        SdpLine::Version{..} => (),
        _ => return Result::Err(SdpParserResult::ParserLineError {
            message: "first line needs to be version number".to_string(),
            line: "".to_string() })
    };
    match lines[1] {
        SdpLine::Origin{..} => (),
        _ => return Result::Err(SdpParserResult::ParserLineError {
            message: "second line needs to be origin".to_string(),
            line: "".to_string() })
    };
    match lines[2] {
        SdpLine::Session{..} => (),
        _ => return Result::Err(SdpParserResult::ParserLineError {
            message: "third line needs to be session".to_string(),
            line: "".to_string() })
    };
    for line in &lines {
    }
    Result::Err(SdpParserResult::ParserLineError {
        message: "foo".to_string(),
        line: "bar".to_string() })
}

pub fn parse_sdp(sdp: &str, fail_on_warning: bool) -> bool {
    if sdp.is_empty() {
        return false;
    }
    let lines = sdp.lines();
    let mut errors: Vec<SdpParserResult> = Vec::new();
    let mut warnings: Vec<SdpParserResult> = Vec::new();
    let mut sdp_lines: Vec<SdpLine> = Vec::new();
    for line in lines {
        match parse_sdp_line(line) {
            Ok(n) => { sdp_lines.push(n); },
            Err(e) => {
                match e {
                    // FIXME is this really a good way to accomplish this?
                    SdpParserResult::ParserLineError { message: x, line: y } =>
                        { errors.push(SdpParserResult::ParserLineError { message: x, line: y}) }
                    SdpParserResult::ParserUnsupported { message: x, line: y } =>
                        {
                            println!("Warning unsupported value encountered: {}\n in line {}", x, y);
                            warnings.push(SdpParserResult::ParserUnsupported { message: x, line: y});
                        }
                }
            }
        };
    };
    let mut ret: bool = true;
    if warnings.len() > 0 {
        while let Some(x) = errors.pop() {
            match x {
                SdpParserResult::ParserLineError { message: msg, line: l} =>
                    { println!("Parser error: {}\n  in line: {}", msg, l) }
                SdpParserResult::ParserUnsupported { message: msg, line: l} =>
                    { println!("Parser unknown: {}\n  in line: {}", msg, l) }
            };
        };
        if fail_on_warning {
            ret = false;
        };
    };
    if errors.len() > 0 {
        while let Some(x) = errors.pop() {
            match x {
                SdpParserResult::ParserLineError { message: msg, line: l} =>
                    { println!("Parser error: {}\n  in line: {}", msg, l) }
                SdpParserResult::ParserUnsupported { message: msg, line: l} =>
                    { println!("Parser unknown: {}\n  in line: {}", msg, l) }
            };
        };
        ret = false;
    };
    ret
}
