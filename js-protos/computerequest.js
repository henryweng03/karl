/**
 * @fileoverview
 * @enhanceable
 * @suppress {messageConventions} JS Compiler reports an error if a variable or
 *     field starts with 'MSG_' and isn't a translatable message.
 * @public
 */
// GENERATED CODE -- DO NOT EDIT!

goog.provide('proto.request.ComputeRequest');

goog.require('jspb.BinaryReader');
goog.require('jspb.BinaryWriter');
goog.require('jspb.Message');
goog.require('proto.request.Import');


/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.request.ComputeRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.request.ComputeRequest.repeatedFields_, null);
};
goog.inherits(proto.request.ComputeRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  proto.request.ComputeRequest.displayName = 'proto.request.ComputeRequest';
}
/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.request.ComputeRequest.repeatedFields_ = [2,3,7,8];



if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto suitable for use in Soy templates.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     com.google.apps.jspb.JsClassTemplate.JS_RESERVED_WORDS.
 * @param {boolean=} opt_includeInstance Whether to include the JSPB instance
 *     for transitional soy proto support: http://goto/soy-param-migration
 * @return {!Object}
 */
proto.request.ComputeRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.request.ComputeRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Whether to include the JSPB
 *     instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.request.ComputeRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.request.ComputeRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    binaryPath: jspb.Message.getFieldWithDefault(msg, 1, ""),
    argsList: jspb.Message.getRepeatedField(msg, 2),
    envsList: jspb.Message.getRepeatedField(msg, 3),
    pb_package: msg.getPackage_asB64(),
    stdout: jspb.Message.getFieldWithDefault(msg, 5, false),
    stderr: jspb.Message.getFieldWithDefault(msg, 6, false),
    filesList: jspb.Message.getRepeatedField(msg, 7),
    importsList: jspb.Message.toObjectList(msg.getImportsList(),
    proto.request.Import.toObject, includeInstance)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.request.ComputeRequest}
 */
proto.request.ComputeRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.request.ComputeRequest;
  return proto.request.ComputeRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.request.ComputeRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.request.ComputeRequest}
 */
proto.request.ComputeRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setBinaryPath(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.addArgs(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.addEnvs(value);
      break;
    case 4:
      var value = /** @type {!Uint8Array} */ (reader.readBytes());
      msg.setPackage(value);
      break;
    case 5:
      var value = /** @type {boolean} */ (reader.readBool());
      msg.setStdout(value);
      break;
    case 6:
      var value = /** @type {boolean} */ (reader.readBool());
      msg.setStderr(value);
      break;
    case 7:
      var value = /** @type {string} */ (reader.readString());
      msg.addFiles(value);
      break;
    case 8:
      var value = new proto.request.Import;
      reader.readMessage(value,proto.request.Import.deserializeBinaryFromReader);
      msg.addImports(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.request.ComputeRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.request.ComputeRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.request.ComputeRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.request.ComputeRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getBinaryPath();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getArgsList();
  if (f.length > 0) {
    writer.writeRepeatedString(
      2,
      f
    );
  }
  f = message.getEnvsList();
  if (f.length > 0) {
    writer.writeRepeatedString(
      3,
      f
    );
  }
  f = message.getPackage_asU8();
  if (f.length > 0) {
    writer.writeBytes(
      4,
      f
    );
  }
  f = message.getStdout();
  if (f) {
    writer.writeBool(
      5,
      f
    );
  }
  f = message.getStderr();
  if (f) {
    writer.writeBool(
      6,
      f
    );
  }
  f = message.getFilesList();
  if (f.length > 0) {
    writer.writeRepeatedString(
      7,
      f
    );
  }
  f = message.getImportsList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      8,
      f,
      proto.request.Import.serializeBinaryToWriter
    );
  }
};


/**
 * optional string binary_path = 1;
 * @return {string}
 */
proto.request.ComputeRequest.prototype.getBinaryPath = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/** @param {string} value */
proto.request.ComputeRequest.prototype.setBinaryPath = function(value) {
  jspb.Message.setField(this, 1, value);
};


/**
 * repeated string args = 2;
 * @return {!Array.<string>}
 */
proto.request.ComputeRequest.prototype.getArgsList = function() {
  return /** @type {!Array.<string>} */ (jspb.Message.getRepeatedField(this, 2));
};


/** @param {!Array.<string>} value */
proto.request.ComputeRequest.prototype.setArgsList = function(value) {
  jspb.Message.setField(this, 2, value || []);
};


/**
 * @param {!string} value
 * @param {number=} opt_index
 */
proto.request.ComputeRequest.prototype.addArgs = function(value, opt_index) {
  jspb.Message.addToRepeatedField(this, 2, value, opt_index);
};


proto.request.ComputeRequest.prototype.clearArgsList = function() {
  this.setArgsList([]);
};


/**
 * repeated string envs = 3;
 * @return {!Array.<string>}
 */
proto.request.ComputeRequest.prototype.getEnvsList = function() {
  return /** @type {!Array.<string>} */ (jspb.Message.getRepeatedField(this, 3));
};


/** @param {!Array.<string>} value */
proto.request.ComputeRequest.prototype.setEnvsList = function(value) {
  jspb.Message.setField(this, 3, value || []);
};


/**
 * @param {!string} value
 * @param {number=} opt_index
 */
proto.request.ComputeRequest.prototype.addEnvs = function(value, opt_index) {
  jspb.Message.addToRepeatedField(this, 3, value, opt_index);
};


proto.request.ComputeRequest.prototype.clearEnvsList = function() {
  this.setEnvsList([]);
};


/**
 * optional bytes package = 4;
 * @return {string}
 */
proto.request.ComputeRequest.prototype.getPackage = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 4, ""));
};


/**
 * optional bytes package = 4;
 * This is a type-conversion wrapper around `getPackage()`
 * @return {string}
 */
proto.request.ComputeRequest.prototype.getPackage_asB64 = function() {
  return /** @type {string} */ (jspb.Message.bytesAsB64(
      this.getPackage()));
};


/**
 * optional bytes package = 4;
 * Note that Uint8Array is not supported on all browsers.
 * @see http://caniuse.com/Uint8Array
 * This is a type-conversion wrapper around `getPackage()`
 * @return {!Uint8Array}
 */
proto.request.ComputeRequest.prototype.getPackage_asU8 = function() {
  return /** @type {!Uint8Array} */ (jspb.Message.bytesAsU8(
      this.getPackage()));
};


/** @param {!(string|Uint8Array)} value */
proto.request.ComputeRequest.prototype.setPackage = function(value) {
  jspb.Message.setField(this, 4, value);
};


/**
 * optional bool stdout = 5;
 * Note that Boolean fields may be set to 0/1 when serialized from a Java server.
 * You should avoid comparisons like {@code val === true/false} in those cases.
 * @return {boolean}
 */
proto.request.ComputeRequest.prototype.getStdout = function() {
  return /** @type {boolean} */ (jspb.Message.getFieldWithDefault(this, 5, false));
};


/** @param {boolean} value */
proto.request.ComputeRequest.prototype.setStdout = function(value) {
  jspb.Message.setField(this, 5, value);
};


/**
 * optional bool stderr = 6;
 * Note that Boolean fields may be set to 0/1 when serialized from a Java server.
 * You should avoid comparisons like {@code val === true/false} in those cases.
 * @return {boolean}
 */
proto.request.ComputeRequest.prototype.getStderr = function() {
  return /** @type {boolean} */ (jspb.Message.getFieldWithDefault(this, 6, false));
};


/** @param {boolean} value */
proto.request.ComputeRequest.prototype.setStderr = function(value) {
  jspb.Message.setField(this, 6, value);
};


/**
 * repeated string files = 7;
 * @return {!Array.<string>}
 */
proto.request.ComputeRequest.prototype.getFilesList = function() {
  return /** @type {!Array.<string>} */ (jspb.Message.getRepeatedField(this, 7));
};


/** @param {!Array.<string>} value */
proto.request.ComputeRequest.prototype.setFilesList = function(value) {
  jspb.Message.setField(this, 7, value || []);
};


/**
 * @param {!string} value
 * @param {number=} opt_index
 */
proto.request.ComputeRequest.prototype.addFiles = function(value, opt_index) {
  jspb.Message.addToRepeatedField(this, 7, value, opt_index);
};


proto.request.ComputeRequest.prototype.clearFilesList = function() {
  this.setFilesList([]);
};


/**
 * repeated Import imports = 8;
 * @return {!Array.<!proto.request.Import>}
 */
proto.request.ComputeRequest.prototype.getImportsList = function() {
  return /** @type{!Array.<!proto.request.Import>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.request.Import, 8));
};


/** @param {!Array.<!proto.request.Import>} value */
proto.request.ComputeRequest.prototype.setImportsList = function(value) {
  jspb.Message.setRepeatedWrapperField(this, 8, value);
};


/**
 * @param {!proto.request.Import=} opt_value
 * @param {number=} opt_index
 * @return {!proto.request.Import}
 */
proto.request.ComputeRequest.prototype.addImports = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 8, opt_value, proto.request.Import, opt_index);
};


proto.request.ComputeRequest.prototype.clearImportsList = function() {
  this.setImportsList([]);
};


