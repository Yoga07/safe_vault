initSidebarItems({"constant":[["GROUP_SIZE","The group size for the routing table. This is the maximum that can be used for consensus."],["MAX_IMMUTABLE_DATA_SIZE_IN_BYTES","Maximum allowed size for a serialised Immutable Data (ID) to grow to"],["MAX_PRIV_APPENDABLE_DATA_SIZE_IN_BYTES","Maximum allowed size for a private appendable data to grow to"],["MAX_PUB_APPENDABLE_DATA_SIZE_IN_BYTES","Maximum allowed size for a public appendable data to grow to"],["MAX_STRUCTURED_DATA_SIZE_IN_BYTES","Maximum allowed size for a Structured Data to grow to"],["NO_OWNER_PUB_KEY","A signing key with no matching private key. Passing ownership to it will make a chunk effectively immutable."],["QUORUM_SIZE","The quorum for group consensus."],["TYPE_TAG_DNS_PACKET","Structured Data Tag for DNS Packet Type"],["TYPE_TAG_SESSION_PACKET","Structured Data Tag for Session Packet Type"],["XOR_NAME_BITS","Constant bit length of `XorName`."],["XOR_NAME_LEN","Constant byte length of `XorName`."]],"enum":[["AppendWrapper","An `AppendedData` item, together with the identifier of the data to append it to."],["Authority","An entity that can act as a source or destination of a message."],["Data","This is the data types routing handles in the public interface"],["DataIdentifier","An identifier to address a data chunk."],["Event","An Event raised by a `Node` or `Client` via its event sender."],["Filter","The type of access filter for appendable data."],["InterfaceError","The type of errors that can occur if routing is unable to handle a send request."],["Request","Request message types"],["Response","Response message types"],["RoutingError","The type of errors that can occur during handling of routing events."],["XorNameFromHexError","Errors that can occur when decoding a `XorName` from a string."]],"mod":[["client_errors","Error communication between vaults and core"],["messaging","Messaging infrastructure"]],"struct":[["AppendedData","An appended data item, pointing to another data chunk in the network."],["Client","Interface for sending and receiving messages to and from a network of nodes in the role of a client."],["FullId","Network identity component containing name, and public and private keys."],["ImmutableData","An immutable chunk of data."],["MessageId","Unique ID for messages"],["Node","Interface for sending and receiving messages to and from other nodes, in the role of a full routing node."],["NodeBuilder","A builder to configure and create a new `Node`."],["PlainData","Plain data with a name and a value."],["PrivAppendableData","Private appendable data."],["PrivAppendedData","A private appended data item: an encrypted `AppendedData`."],["PubAppendableData","Public appendable data."],["PublicId","Network identity component containing name and public keys."],["StructuredData","Mutable structured data."],["XorName","A `XOR_NAME_BITS`-bit number, viewed as a point in XOR space."]],"trait":[["Cache","A cache that stores `Response`s keyed by `Requests`. Should be implemented by layers above routing."]]});