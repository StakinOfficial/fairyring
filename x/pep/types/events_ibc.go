package types

// IBC events
const (
	EventTypeTimeout = "timeout"
	// this line is used by starport scaffolding # ibc/packet/event

	AttributeKeyAckSuccess = "success"
	AttributeKeyAck        = "acknowledgement"
	AttributeKeyAckError   = "error"

	EventTypeCurrentKeysPacket = "currentKeys_packet"
	EventTypeRequestKeyshare   = "keyshare_request_packet"

	AttributeKeyRequestID = "request_id"
	AttributeKeyCreator   = "creator"
)
