#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn gnuk_malloc(_: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn gnuk_free(_: *mut libc::c_void);
    /* NOTE: This signature is different to PTHREAD's one.  */
    #[no_mangle]
    fn chopstx_setcancelstate(_: libc::c_int) -> libc::c_int;
    /* NOTE: This signature is different to PTHREAD's one.  */
    #[no_mangle]
    fn chopstx_cleanup_push(clp_0: *mut chopstx_cleanup_t);
    #[no_mangle]
    fn chopstx_cleanup_pop(execute: libc::c_int);
    #[no_mangle]
    fn set_res_sw(sw1: uint8_t, sw2: uint8_t);
    /* iterator returning a byta at a time */
    #[no_mangle]
    fn random_gen(arg: *mut libc::c_void, output: *mut libc::c_uchar,
                  output_len: size_t) -> libc::c_int;
    /* *
 * \file rsa.h
 *
 * \brief The RSA public-key cryptosystem
 *
 *  Copyright (C) 2006-2010, Brainspark B.V.
 *
 *  This file is part of PolarSSL (http://www.polarssl.org)
 *  Lead Maintainer: Paul Bakker <polarssl_maintainer at polarssl.org>
 *
 *  All rights reserved.
 *
 *  This program is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation; either version 2 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License along
 *  with this program; if not, write to the Free Software Foundation, Inc.,
 *  51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
 */
    /*
 * RSA Error codes
 */
    /* *< Bad input parameters to function. */
    /* *< Input data contains invalid padding and is rejected. */
    /* *< Something failed during generation of a key. */
    /* *< Key failed to pass the libraries validity check. */
    /* *< The public key operation failed. */
    /* *< The private key operation failed. */
    /* *< The PKCS#1 verification failed. */
    /* *< The output buffer for decryption is not large enough. */
    /* *< The random generator failed to generate non-zeros. */
    /*
 * PKCS#1 constants
 */
    /*
 * ISO Member bodies OID parts
 */
    /*
 * ISO Identified organization OID parts
 */
    /*
 * DigestInfo ::= SEQUENCE {
 *   digestAlgorithm DigestAlgorithmIdentifier,
 *   digest Digest }
 *
 * DigestAlgorithmIdentifier ::= AlgorithmIdentifier
 *
 * Digest ::= OCTET STRING
 */
    /* *
 * \brief          RSA context structure
 */
    /* !<  always 0          */
    /* !<  size(N) in chars  */
    /* !<  public modulus    */
    /* !<  public exponent   */
    /* !<  private exponent  */
    /* !<  1st prime factor  */
    /* !<  2nd prime factor  */
    /* !<  D % (P - 1)       */
    /* !<  D % (Q - 1)       */
    /* !<  1 / (Q % P)       */
    /* !<  cached R^2 mod N  */
    /* !<  cached R^2 mod P  */
    /* !<  cached R^2 mod Q  */
    /* !<  RSA_PKCS_V15 for 1.5 padding and
                                      RSA_PKCS_v21 for OAEP/PSS         */
    /* !<  Hash identifier of md_type_t as
                                      specified in the md.h header file
                                      for the EME-OAEP and EMSA-PSS
                                      encoding                          */
    /* *
 * \brief          Initialize an RSA context
 *
 *                 Note: Set padding to RSA_PKCS_V21 for the RSAES-OAEP
 *                 encryption scheme and the RSASSA-PSS signature scheme.
 *
 * \param ctx      RSA context to be initialized
 * \param padding  RSA_PKCS_V15 or RSA_PKCS_V21
 * \param hash_id  RSA_PKCS_V21 hash identifier
 *
 * \note           The hash_id parameter is actually ignored
 *                 when using RSA_PKCS_V15 padding.
 */
    /* *
 * \brief          Generate an RSA keypair
 *
 * \param ctx      RSA context that will hold the key
 * \param f_rng    RNG function
 * \param p_rng    RNG parameter
 * \param nbits    size of the public key in bits
 * \param exponent public exponent (e.g., 65537)
 *
 * \note           rsa_init() must be called beforehand to setup
 *                 the RSA context.
 *
 * \return         0 if successful, or an POLARSSL_ERR_RSA_XXX error code
 */
    /* *
 * \brief          Check a public RSA key
 *
 * \param ctx      RSA context to be checked
 *
 * \return         0 if successful, or an POLARSSL_ERR_RSA_XXX error code
 */
    /* *
 * \brief          Check a private RSA key
 *
 * \param ctx      RSA context to be checked
 *
 * \return         0 if successful, or an POLARSSL_ERR_RSA_XXX error code
 */
    /* *
 * \brief          Do an RSA public key operation
 *
 * \param ctx      RSA context
 * \param input    input buffer
 * \param output   output buffer
 *
 * \return         0 if successful, or an POLARSSL_ERR_RSA_XXX error code
 *
 * \note           This function does NOT take care of message
 *                 padding. Also, be sure to set input[0] = 0 or assure that
 *                 input is smaller than N.
 *
 * \note           The input and output buffers must be large
 *                 enough (eg. 128 bytes if RSA-1024 is used).
 */
    /* *
 * \brief          Do an RSA private key operation
 *
 * \param ctx      RSA context
 * \param f_rng    RNG function (Needed for blinding)
 * \param p_rng    RNG parameter
 * \param input    input buffer
 * \param output   output buffer
 *
 * \return         0 if successful, or an POLARSSL_ERR_RSA_XXX error code
 *
 * \note           The input and output buffers must be large
 *                 enough (eg. 128 bytes if RSA-1024 is used).
 */
    /* *
 * \brief          Generic wrapper to perform a PKCS#1 encryption using the
 *                 mode from the context. Add the message padding, then do an
 *                 RSA operation.
 *
 * \param ctx      RSA context
 * \param f_rng    RNG function (Needed for padding and PKCS#1 v2.1 encoding
 *                               and RSA_PRIVATE)
 * \param p_rng    RNG parameter
 * \param mode     RSA_PUBLIC or RSA_PRIVATE
 * \param ilen     contains the plaintext length
 * \param input    buffer holding the data to be encrypted
 * \param output   buffer that will hold the ciphertext
 *
 * \return         0 if successful, or an POLARSSL_ERR_RSA_XXX error code
 *
 * \note           The output buffer must be as large as the size
 *                 of ctx->N (eg. 128 bytes if RSA-1024 is used).
 */
    /* *
 * \brief          Perform a PKCS#1 v1.5 encryption (RSAES-PKCS1-v1_5-ENCRYPT)
 *
 * \param ctx      RSA context
 * \param f_rng    RNG function (Needed for padding and RSA_PRIVATE)
 * \param p_rng    RNG parameter
 * \param mode     RSA_PUBLIC or RSA_PRIVATE
 * \param ilen     contains the plaintext length
 * \param input    buffer holding the data to be encrypted
 * \param output   buffer that will hold the ciphertext
 *
 * \return         0 if successful, or an POLARSSL_ERR_RSA_XXX error code
 *
 * \note           The output buffer must be as large as the size
 *                 of ctx->N (eg. 128 bytes if RSA-1024 is used).
 */
    /* *
 * \brief          Perform a PKCS#1 v2.1 OAEP encryption (RSAES-OAEP-ENCRYPT)
 *
 * \param ctx      RSA context
 * \param f_rng    RNG function (Needed for padding and PKCS#1 v2.1 encoding
 *                               and RSA_PRIVATE)
 * \param p_rng    RNG parameter
 * \param mode     RSA_PUBLIC or RSA_PRIVATE
 * \param label    buffer holding the custom label to use
 * \param label_len contains the label length
 * \param ilen     contains the plaintext length
 * \param input    buffer holding the data to be encrypted
 * \param output   buffer that will hold the ciphertext
 *
 * \return         0 if successful, or an POLARSSL_ERR_RSA_XXX error code
 *
 * \note           The output buffer must be as large as the size
 *                 of ctx->N (eg. 128 bytes if RSA-1024 is used).
 */
    /* *
 * \brief          Generic wrapper to perform a PKCS#1 decryption using the
 *                 mode from the context. Do an RSA operation, then remove
 *                 the message padding
 *
 * \param ctx      RSA context
 * \param f_rng    RNG function (Only needed for RSA_PRIVATE)
 * \param p_rng    RNG parameter
 * \param mode     RSA_PUBLIC or RSA_PRIVATE
 * \param olen     will contain the plaintext length
 * \param input    buffer holding the encrypted data
 * \param output   buffer that will hold the plaintext
 * \param output_max_len    maximum length of the output buffer
 *
 * \return         0 if successful, or an POLARSSL_ERR_RSA_XXX error code
 *
 * \note           The output buffer must be as large as the size
 *                 of ctx->N (eg. 128 bytes if RSA-1024 is used) otherwise
 *                 an error is thrown.
 */
    /* *
 * \brief          Perform a PKCS#1 v1.5 decryption (RSAES-PKCS1-v1_5-DECRYPT)
 *
 * \param ctx      RSA context
 * \param f_rng    RNG function (Only needed for RSA_PRIVATE)
 * \param p_rng    RNG parameter
 * \param mode     RSA_PUBLIC or RSA_PRIVATE
 * \param olen     will contain the plaintext length
 * \param input    buffer holding the encrypted data
 * \param output   buffer that will hold the plaintext
 * \param output_max_len    maximum length of the output buffer
 *
 * \return         0 if successful, or an POLARSSL_ERR_RSA_XXX error code
 *
 * \note           The output buffer must be as large as the size
 *                 of ctx->N (eg. 128 bytes if RSA-1024 is used) otherwise
 *                 an error is thrown.
 */
    /* *
 * \brief          Perform a PKCS#1 v2.1 OAEP decryption (RSAES-OAEP-DECRYPT)
 *
 * \param ctx      RSA context
 * \param f_rng    RNG function (Only needed for RSA_PRIVATE)
 * \param p_rng    RNG parameter
 * \param mode     RSA_PUBLIC or RSA_PRIVATE
 * \param label    buffer holding the custom label to use
 * \param label_len contains the label length
 * \param olen     will contain the plaintext length
 * \param input    buffer holding the encrypted data
 * \param output   buffer that will hold the plaintext
 * \param output_max_len    maximum length of the output buffer
 *
 * \return         0 if successful, or an POLARSSL_ERR_RSA_XXX error code
 *
 * \note           The output buffer must be as large as the size
 *                 of ctx->N (eg. 128 bytes if RSA-1024 is used) otherwise
 *                 an error is thrown.
 */
    /* *
 * \brief          Generic wrapper to perform a PKCS#1 signature using the
 *                 mode from the context. Do a private RSA operation to sign
 *                 a message digest
 *
 * \param ctx      RSA context
 * \param f_rng    RNG function (Needed for PKCS#1 v2.1 encoding and for
 *                 RSA_PRIVATE)
 * \param p_rng    RNG parameter
 * \param mode     RSA_PUBLIC or RSA_PRIVATE
 * \param hash_id  SIG_RSA_RAW, SIG_RSA_MD{2,4,5} or SIG_RSA_SHA{1,224,256,384,512}
 * \param hashlen  message digest length (for SIG_RSA_RAW only)
 * \param hash     buffer holding the message digest
 * \param sig      buffer that will hold the ciphertext
 *
 * \return         0 if the signing operation was successful,
 *                 or an POLARSSL_ERR_RSA_XXX error code
 *
 * \note           The "sig" buffer must be as large as the size
 *                 of ctx->N (eg. 128 bytes if RSA-1024 is used).
 *
 * \note           In case of PKCS#1 v2.1 encoding keep in mind that
 *                 the hash_id in the RSA context is the one used for the
 *                 encoding. hash_id in the function call is the type of hash
 *                 that is encoded. According to RFC 3447 it is advised to
 *                 keep both hashes the same.
 */
    /* *
 * \brief          Perform a PKCS#1 v1.5 signature (RSASSA-PKCS1-v1_5-SIGN)
 *
 * \param ctx      RSA context
 * \param f_rng    RNG function (Only needed for RSA_PRIVATE)
 * \param p_rng    RNG parameter
 * \param mode     RSA_PUBLIC or RSA_PRIVATE
 * \param hash_id  SIG_RSA_RAW, SIG_RSA_MD{2,4,5} or SIG_RSA_SHA{1,224,256,384,512}
 * \param hashlen  message digest length (for SIG_RSA_RAW only)
 * \param hash     buffer holding the message digest
 * \param sig      buffer that will hold the ciphertext
 *
 * \return         0 if the signing operation was successful,
 *                 or an POLARSSL_ERR_RSA_XXX error code
 *
 * \note           The "sig" buffer must be as large as the size
 *                 of ctx->N (eg. 128 bytes if RSA-1024 is used).
 */
    /* *
 * \brief          Perform a PKCS#1 v2.1 PSS signature (RSASSA-PSS-SIGN)
 *
 * \param ctx      RSA context
 * \param f_rng    RNG function (Needed for PKCS#1 v2.1 encoding and for
 *                               RSA_PRIVATE)
 * \param p_rng    RNG parameter
 * \param mode     RSA_PUBLIC or RSA_PRIVATE
 * \param hash_id  SIG_RSA_RAW, SIG_RSA_MD{2,4,5} or SIG_RSA_SHA{1,224,256,384,512}
 * \param hashlen  message digest length (for SIG_RSA_RAW only)
 * \param hash     buffer holding the message digest
 * \param sig      buffer that will hold the ciphertext
 *
 * \return         0 if the signing operation was successful,
 *                 or an POLARSSL_ERR_RSA_XXX error code
 *
 * \note           The "sig" buffer must be as large as the size
 *                 of ctx->N (eg. 128 bytes if RSA-1024 is used).
 *
 * \note           In case of PKCS#1 v2.1 encoding keep in mind that
 *                 the hash_id in the RSA context is the one used for the
 *                 encoding. hash_id in the function call is the type of hash
 *                 that is encoded. According to RFC 3447 it is advised to
 *                 keep both hashes the same.
 */
    /* *
 * \brief          Generic wrapper to perform a PKCS#1 verification using the
 *                 mode from the context. Do a public RSA operation and check
 *                 the message digest
 *
 * \param ctx      points to an RSA public key
 * \param f_rng    RNG function (Only needed for RSA_PRIVATE)
 * \param p_rng    RNG parameter
 * \param mode     RSA_PUBLIC or RSA_PRIVATE
 * \param hash_id  SIG_RSA_RAW, SIG_RSA_MD{2,4,5} or SIG_RSA_SHA{1,224,256,384,512}
 * \param hashlen  message digest length (for SIG_RSA_RAW only)
 * \param hash     buffer holding the message digest
 * \param sig      buffer holding the ciphertext
 *
 * \return         0 if the verify operation was successful,
 *                 or an POLARSSL_ERR_RSA_XXX error code
 *
 * \note           The "sig" buffer must be as large as the size
 *                 of ctx->N (eg. 128 bytes if RSA-1024 is used).
 *
 * \note           In case of PKCS#1 v2.1 encoding keep in mind that
 *                 the hash_id in the RSA context is the one used for the
 *                 verification. hash_id in the function call is the type of hash
 *                 that is verified. According to RFC 3447 it is advised to
 *                 keep both hashes the same.
 */
    /* *
 * \brief          Perform a PKCS#1 v1.5 verification (RSASSA-PKCS1-v1_5-VERIFY)
 *
 * \param ctx      points to an RSA public key
 * \param f_rng    RNG function (Only needed for RSA_PRIVATE)
 * \param p_rng    RNG parameter
 * \param mode     RSA_PUBLIC or RSA_PRIVATE
 * \param hash_id  SIG_RSA_RAW, SIG_RSA_MD{2,4,5} or SIG_RSA_SHA{1,224,256,384,512}
 * \param hashlen  message digest length (for SIG_RSA_RAW only)
 * \param hash     buffer holding the message digest
 * \param sig      buffer holding the ciphertext
 *
 * \return         0 if the verify operation was successful,
 *                 or an POLARSSL_ERR_RSA_XXX error code
 *
 * \note           The "sig" buffer must be as large as the size
 *                 of ctx->N (eg. 128 bytes if RSA-1024 is used).
 */
    /* *
 * \brief          Perform a PKCS#1 v2.1 PSS verification (RSASSA-PSS-VERIFY)
 * \brief          Do a public RSA and check the message digest
 *
 * \param ctx      points to an RSA public key
 * \param f_rng    RNG function (Only needed for RSA_PRIVATE)
 * \param p_rng    RNG parameter
 * \param mode     RSA_PUBLIC or RSA_PRIVATE
 * \param hash_id  SIG_RSA_RAW, SIG_RSA_MD{2,4,5} or SIG_RSA_SHA{1,224,256,384,512}
 * \param hashlen  message digest length (for SIG_RSA_RAW only)
 * \param hash     buffer holding the message digest
 * \param sig      buffer holding the ciphertext
 *
 * \return         0 if the verify operation was successful,
 *                 or an POLARSSL_ERR_RSA_XXX error code
 *
 * \note           The "sig" buffer must be as large as the size
 *                 of ctx->N (eg. 128 bytes if RSA-1024 is used).
 *
 * \note           In case of PKCS#1 v2.1 encoding keep in mind that
 *                 the hash_id in the RSA context is the one used for the
 *                 verification. hash_id in the function call is the type of hash
 *                 that is verified. According to RFC 3447 it is advised to
 *                 keep both hashes the same.
 */
    /* *
 * \brief          Free the components of an RSA key
 *
 * \param ctx      RSA Context to free
 */
    #[no_mangle]
    fn rsa_free(ctx: *mut rsa_context);
    #[no_mangle]
    fn rsa_gen_key(ctx: *mut rsa_context,
                   f_rng:
                       Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                   _: *mut libc::c_uchar,
                                                   _: size_t) -> libc::c_int>,
                   p_rng: *mut libc::c_void, nbits: libc::c_uint,
                   exponent: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn rsa_rsassa_pkcs1_v15_verify(ctx: *mut rsa_context,
                                   f_rng:
                                       Option<unsafe extern "C" fn(_:
                                                                       *mut libc::c_void,
                                                                   _:
                                                                       *mut libc::c_uchar,
                                                                   _: size_t)
                                                  -> libc::c_int>,
                                   p_rng: *mut libc::c_void,
                                   mode: libc::c_int, hash_id: libc::c_int,
                                   hashlen: libc::c_uint,
                                   hash: *const libc::c_uchar,
                                   sig: *const libc::c_uchar) -> libc::c_int;
    #[no_mangle]
    fn rsa_rsaes_pkcs1_v15_decrypt(ctx: *mut rsa_context,
                                   f_rng:
                                       Option<unsafe extern "C" fn(_:
                                                                       *mut libc::c_void,
                                                                   _:
                                                                       *mut libc::c_uchar,
                                                                   _: size_t)
                                                  -> libc::c_int>,
                                   p_rng: *mut libc::c_void,
                                   mode: libc::c_int, olen: *mut size_t,
                                   input: *const libc::c_uchar,
                                   output: *mut libc::c_uchar,
                                   output_max_len: size_t) -> libc::c_int;
    /* *
 * \brief           Initialize one MPI
 *
 * \param X         One MPI to initialize.
 */
    /* *
 * \brief          Unallocate one MPI
 *
 * \param X        One MPI to unallocate.
 */
    /* *
 * \brief          Enlarge to the specified number of limbs
 *
 * \param X        MPI to grow
 * \param nblimbs  The target number of limbs
 *
 * \return         0 if successful,
 *                 POLARSSL_ERR_MPI_MALLOC_FAILED if memory allocation failed
 */
    /* *
 * \brief          Copy the contents of Y into X
 *
 * \param X        Destination MPI
 * \param Y        Source MPI
 *
 * \return         0 if successful,
 *                 POLARSSL_ERR_MPI_MALLOC_FAILED if memory allocation failed
 */
    /* *
 * \brief          Swap the contents of X and Y
 *
 * \param X        First MPI value
 * \param Y        Second MPI value
 */
    /* *
 * \brief          Set value from integer
 *
 * \param X        MPI to set
 * \param z        Value to use
 *
 * \return         0 if successful,
 *                 POLARSSL_ERR_MPI_MALLOC_FAILED if memory allocation failed
 */
    /* *
 * \brief          Get a specific bit from X
 *
 * \param X        MPI to use
 * \param pos      Zero-based index of the bit in X
 *
 * \return         Either a 0 or a 1
 */
    /* *
 * \brief          Set a bit of X to a specific value of 0 or 1
 *
 * \note           Will grow X if necessary to set a bit to 1 in a not yet
 *                 existing limb. Will not grow if bit should be set to 0
 *
 * \param X        MPI to use
 * \param pos      Zero-based index of the bit in X
 * \param val      The value to set the bit to (0 or 1)
 *
 * \return         0 if successful,
 *                 POLARSSL_ERR_MPI_MALLOC_FAILED if memory allocation failed,
 *                 POLARSSL_ERR_MPI_BAD_INPUT_DATA if val is not 0 or 1
 */
    /* *
 * \brief          Return the number of zero-bits before the least significant
 *                 '1' bit
 *
 * Note: Thus also the zero-based index of the least significant '1' bit
 *
 * \param X        MPI to use
 */
    /* *
 * \brief          Return the number of bits up to and including the most
 *                 significant '1' bit'
 *
 * Note: Thus also the one-based index of the most significant '1' bit
 *
 * \param X        MPI to use
 */
    /* *
 * \brief          Return the total size in bytes
 *
 * \param X        MPI to use
 */
    /* *
 * \brief          Import from an ASCII string
 *
 * \param X        Destination MPI
 * \param radix    Input numeric base
 * \param s        Null-terminated string buffer
 *
 * \return         0 if successful, or a POLARSSL_ERR_MPI_XXX error code
 */
    /* *
 * \brief          Export into an ASCII string
 *
 * \param X        Source MPI
 * \param radix    Output numeric base
 * \param s        String buffer
 * \param slen     String buffer size
 *
 * \return         0 if successful, or a POLARSSL_ERR_MPI_XXX error code.
 *                 *slen is always updated to reflect the amount
 *                 of data that has (or would have) been written.
 *
 * \note           Call this function with *slen = 0 to obtain the
 *                 minimum required buffer size in *slen.
 */
    /* POLARSSL_FS_IO */
    /* *
 * \brief          Import X from unsigned binary data, big endian
 *
 * \param X        Destination MPI
 * \param buf      Input buffer
 * \param buflen   Input buffer size
 *
 * \return         0 if successful,
 *                 POLARSSL_ERR_MPI_MALLOC_FAILED if memory allocation failed
 */
    /* *
 * \brief          Export X into unsigned binary data, big endian
 *
 * \param X        Source MPI
 * \param buf      Output buffer
 * \param buflen   Output buffer size
 *
 * \return         0 if successful,
 *                 POLARSSL_ERR_MPI_BUFFER_TOO_SMALL if buf isn't large enough
 */
    #[no_mangle]
    fn mpi_write_binary(X: *const mpi, buf: *mut libc::c_uchar,
                        buflen: size_t) -> libc::c_int;
    #[no_mangle]
    fn rsa_init(ctx: *mut rsa_context, padding: libc::c_int,
                hash_id: libc::c_int);
    #[no_mangle]
    fn mpi_init(X: *mut mpi);
    #[no_mangle]
    fn mpi_lset(X: *mut mpi, z: t_sint) -> libc::c_int;
    #[no_mangle]
    fn mpi_read_binary(X: *mut mpi, buf: *const libc::c_uchar, buflen: size_t)
     -> libc::c_int;
    /* *
 * \brief          Left-shift: X <<= count
 *
 * \param X        MPI to shift
 * \param count    Amount to shift
 *
 * \return         0 if successful,
 *                 POLARSSL_ERR_MPI_MALLOC_FAILED if memory allocation failed
 */
    /* *
 * \brief          Right-shift: X >>= count
 *
 * \param X        MPI to shift
 * \param count    Amount to shift
 *
 * \return         0 if successful,
 *                 POLARSSL_ERR_MPI_MALLOC_FAILED if memory allocation failed
 */
    /* *
 * \brief          Compare unsigned values
 *
 * \param X        Left-hand MPI
 * \param Y        Right-hand MPI
 *
 * \return         1 if |X| is greater than |Y|,
 *                -1 if |X| is lesser  than |Y| or
 *                 0 if |X| is equal to |Y|
 */
    /* *
 * \brief          Compare signed values
 *
 * \param X        Left-hand MPI
 * \param Y        Right-hand MPI
 *
 * \return         1 if X is greater than Y,
 *                -1 if X is lesser  than Y or
 *                 0 if X is equal to Y
 */
    /* *
 * \brief          Compare signed values
 *
 * \param X        Left-hand MPI
 * \param z        The integer value to compare to
 *
 * \return         1 if X is greater than z,
 *                -1 if X is lesser  than z or
 *                 0 if X is equal to z
 */
    /* *
 * \brief          Unsigned addition: X = |A| + |B|
 *
 * \param X        Destination MPI
 * \param A        Left-hand MPI
 * \param B        Right-hand MPI
 *
 * \return         0 if successful,
 *                 POLARSSL_ERR_MPI_MALLOC_FAILED if memory allocation failed
 */
    /* *
 * \brief          Unsigned substraction: X = |A| - |B|
 *
 * \param X        Destination MPI
 * \param A        Left-hand MPI
 * \param B        Right-hand MPI
 *
 * \return         0 if successful,
 *                 POLARSSL_ERR_MPI_NEGATIVE_VALUE if B is greater than A
 */
    /* *
 * \brief          Signed addition: X = A + B
 *
 * \param X        Destination MPI
 * \param A        Left-hand MPI
 * \param B        Right-hand MPI
 *
 * \return         0 if successful,
 *                 POLARSSL_ERR_MPI_MALLOC_FAILED if memory allocation failed
 */
    /* *
 * \brief          Signed substraction: X = A - B
 *
 * \param X        Destination MPI
 * \param A        Left-hand MPI
 * \param B        Right-hand MPI
 *
 * \return         0 if successful,
 *                 POLARSSL_ERR_MPI_MALLOC_FAILED if memory allocation failed
 */
    /* *
 * \brief          Signed addition: X = A + b
 *
 * \param X        Destination MPI
 * \param A        Left-hand MPI
 * \param b        The integer value to add
 *
 * \return         0 if successful,
 *                 POLARSSL_ERR_MPI_MALLOC_FAILED if memory allocation failed
 */
    /* *
 * \brief          Signed substraction: X = A - b
 *
 * \param X        Destination MPI
 * \param A        Left-hand MPI
 * \param b        The integer value to subtract
 *
 * \return         0 if successful,
 *                 POLARSSL_ERR_MPI_MALLOC_FAILED if memory allocation failed
 */
    #[no_mangle]
    fn mpi_sub_int(X: *mut mpi, A: *const mpi, b: t_sint) -> libc::c_int;
    /* *
 * \brief          Baseline multiplication: X = A * B
 *
 * \param X        Destination MPI
 * \param A        Left-hand MPI
 * \param B        Right-hand MPI
 *
 * \return         0 if successful,
 *                 POLARSSL_ERR_MPI_MALLOC_FAILED if memory allocation failed
 */
    #[no_mangle]
    fn mpi_mul_mpi(X: *mut mpi, A: *const mpi, B: *const mpi) -> libc::c_int;
    /* *
 * \brief          Baseline multiplication: X = A * b
 *                 Note: b is an unsigned integer type, thus
 *                 Negative values of b are ignored.
 *
 * \param X        Destination MPI
 * \param A        Left-hand MPI
 * \param b        The integer value to multiply with
 *
 * \return         0 if successful,
 *                 POLARSSL_ERR_MPI_MALLOC_FAILED if memory allocation failed
 */
    /* *
 * \brief          Division by mpi: A = Q * B + R
 *
 * \param Q        Destination MPI for the quotient
 * \param R        Destination MPI for the rest value
 * \param A        Left-hand MPI
 * \param B        Right-hand MPI
 *
 * \return         0 if successful,
 *                 POLARSSL_ERR_MPI_MALLOC_FAILED if memory allocation failed,
 *                 POLARSSL_ERR_MPI_DIVISION_BY_ZERO if B == 0
 *
 * \note           Either Q or R can be NULL.
 */
    /* *
 * \brief          Division by int: A = Q * b + R
 *
 * \param Q        Destination MPI for the quotient
 * \param R        Destination MPI for the rest value
 * \param A        Left-hand MPI
 * \param b        Integer to divide by
 *
 * \return         0 if successful,
 *                 POLARSSL_ERR_MPI_MALLOC_FAILED if memory allocation failed,
 *                 POLARSSL_ERR_MPI_DIVISION_BY_ZERO if b == 0
 *
 * \note           Either Q or R can be NULL.
 */
    /* *
 * \brief          Modulo: R = A mod B
 *
 * \param R        Destination MPI for the rest value
 * \param A        Left-hand MPI
 * \param B        Right-hand MPI
 *
 * \return         0 if successful,
 *                 POLARSSL_ERR_MPI_MALLOC_FAILED if memory allocation failed,
 *                 POLARSSL_ERR_MPI_DIVISION_BY_ZERO if B == 0,
 *                 POLARSSL_ERR_MPI_NEGATIVE_VALUE if B < 0
 */
    #[no_mangle]
    fn mpi_mod_mpi(R: *mut mpi, A: *const mpi, B: *const mpi) -> libc::c_int;
    /* *
 * \brief          Modulo: r = A mod b
 *
 * \param r        Destination t_uint
 * \param A        Left-hand MPI
 * \param b        Integer to divide by
 *
 * \return         0 if successful,
 *                 POLARSSL_ERR_MPI_MALLOC_FAILED if memory allocation failed,
 *                 POLARSSL_ERR_MPI_DIVISION_BY_ZERO if b == 0,
 *                 POLARSSL_ERR_MPI_NEGATIVE_VALUE if b < 0
 */
    /* *
 * \brief          Sliding-window exponentiation: X = A^E mod N
 *
 * \param X        Destination MPI 
 * \param A        Left-hand MPI
 * \param E        Exponent MPI
 * \param N        Modular MPI
 * \param _RR      Speed-up MPI used for recalculations
 *
 * \return         0 if successful,
 *                 POLARSSL_ERR_MPI_MALLOC_FAILED if memory allocation failed,
 *                 POLARSSL_ERR_MPI_BAD_INPUT_DATA if N is negative or even or if
 *                 E is negative
 *
 * \note           _RR is used to avoid re-computing R*R mod N across
 *                 multiple calls, which speeds up things a bit. It can
 *                 be set to NULL if the extra performance is unneeded.
 */
    /* *
 * \brief          Fill an MPI X with size bytes of random
 *
 * \param X        Destination MPI
 * \param size     Size in bytes
 * \param f_rng    RNG function
 * \param p_rng    RNG parameter
 *
 * \return         0 if successful,
 *                 POLARSSL_ERR_MPI_MALLOC_FAILED if memory allocation failed
 */
    /* *
 * \brief          Greatest common divisor: G = gcd(A, B)
 *
 * \param G        Destination MPI
 * \param A        Left-hand MPI
 * \param B        Right-hand MPI
 *
 * \return         0 if successful,
 *                 POLARSSL_ERR_MPI_MALLOC_FAILED if memory allocation failed
 */
    /* *
 * \brief          Modular inverse: X = A^-1 mod N
 *
 * \param X        Destination MPI
 * \param A        Left-hand MPI
 * \param N        Right-hand MPI
 *
 * \return         0 if successful,
 *                 POLARSSL_ERR_MPI_MALLOC_FAILED if memory allocation failed,
 *                 POLARSSL_ERR_MPI_BAD_INPUT_DATA if N is negative or nil
                   POLARSSL_ERR_MPI_NOT_ACCEPTABLE if A has no inverse mod N
 */
    #[no_mangle]
    fn mpi_inv_mod(X: *mut mpi, A: *const mpi, N: *const mpi) -> libc::c_int;
    #[no_mangle]
    fn mpi_free(X: *mut mpi);
    #[no_mangle]
    fn rsa_rsassa_pkcs1_v15_sign(ctx: *mut rsa_context,
                                 f_rng:
                                     Option<unsafe extern "C" fn(_:
                                                                     *mut libc::c_void,
                                                                 _:
                                                                     *mut libc::c_uchar,
                                                                 _: size_t)
                                                -> libc::c_int>,
                                 p_rng: *mut libc::c_void, mode: libc::c_int,
                                 hash_id: libc::c_int, hashlen: libc::c_uint,
                                 hash: *const libc::c_uchar,
                                 sig: *mut libc::c_uchar) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chx_cleanup {
    pub next: *mut chx_cleanup,
    pub routine: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    pub arg: *mut libc::c_void,
}
pub type chopstx_cleanup_t = chx_cleanup;
/* RSA-2048 (p and q) */
/* Maximum is the case for RSA 4096-bit.  */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct key_data {
    pub pubkey: *const uint8_t,
    pub data: [uint8_t; 512],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rsa_context {
    pub ver: libc::c_int,
    pub len: size_t,
    pub N: mpi,
    pub E: mpi,
    pub D: mpi,
    pub P: mpi,
    pub Q: mpi,
    pub DP: mpi,
    pub DQ: mpi,
    pub QP: mpi,
    pub RN: mpi,
    pub RP: mpi,
    pub RQ: mpi,
    pub padding: libc::c_int,
    pub hash_id: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpi {
    pub s: libc::c_int,
    pub n: size_t,
    pub p: *mut t_uint,
}
pub type t_uint = uint64_t;
pub type t_sint = int64_t;
/*
 * call-rsa.c -- Glue code between RSA computation and OpenPGP card protocol
 *
 * Copyright (C) 2010, 2011, 2012, 2013, 2014, 2015
 *               Free Software Initiative of Japan
 * Author: NIIBE Yutaka <gniibe@fsij.org>
 *
 * This file is a part of Gnuk, a GnuPG USB Token implementation.
 *
 * Gnuk is free software: you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Gnuk is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
 * or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public
 * License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 */
static mut rsa_ctx: rsa_context =
    rsa_context{ver: 0,
                len: 0,
                N: mpi{s: 0, n: 0, p: 0 as *const t_uint as *mut t_uint,},
                E: mpi{s: 0, n: 0, p: 0 as *const t_uint as *mut t_uint,},
                D: mpi{s: 0, n: 0, p: 0 as *const t_uint as *mut t_uint,},
                P: mpi{s: 0, n: 0, p: 0 as *const t_uint as *mut t_uint,},
                Q: mpi{s: 0, n: 0, p: 0 as *const t_uint as *mut t_uint,},
                DP: mpi{s: 0, n: 0, p: 0 as *const t_uint as *mut t_uint,},
                DQ: mpi{s: 0, n: 0, p: 0 as *const t_uint as *mut t_uint,},
                QP: mpi{s: 0, n: 0, p: 0 as *const t_uint as *mut t_uint,},
                RN: mpi{s: 0, n: 0, p: 0 as *const t_uint as *mut t_uint,},
                RP: mpi{s: 0, n: 0, p: 0 as *const t_uint as *mut t_uint,},
                RQ: mpi{s: 0, n: 0, p: 0 as *const t_uint as *mut t_uint,},
                padding: 0,
                hash_id: 0,};
static mut clp: chx_cleanup =
    chx_cleanup{next: 0 as *const chx_cleanup as *mut chx_cleanup,
                routine: None,
                arg: 0 as *const libc::c_void as *mut libc::c_void,};
unsafe extern "C" fn rsa_cleanup(mut arg: *mut libc::c_void) {
    gnuk_free(arg); /* Allow cancellation.  */
    rsa_free(&mut rsa_ctx);
}
#[no_mangle]
pub unsafe extern "C" fn rsa_sign(mut raw_message: *const uint8_t,
                                  mut output: *mut uint8_t,
                                  mut msg_len: libc::c_int,
                                  mut kd: *mut key_data,
                                  mut pubkey_len: libc::c_int)
 -> libc::c_int {
    let mut P1: mpi = mpi{s: 0, n: 0, p: 0 as *const t_uint as *mut t_uint,};
    let mut Q1: mpi = mpi{s: 0, n: 0, p: 0 as *const t_uint as *mut t_uint,};
    let mut H: mpi = mpi{s: 0, n: 0, p: 0 as *const t_uint as *mut t_uint,};
    let mut ret: libc::c_int = 0 as libc::c_int;
    let vla = pubkey_len as usize;
    let mut temp: Vec<libc::c_uchar> = ::std::vec::from_elem(0, vla);
    rsa_init(&mut rsa_ctx, 0 as libc::c_int, 0 as libc::c_int);
    mpi_init(&mut P1);
    mpi_init(&mut Q1);
    mpi_init(&mut H);
    rsa_ctx.len = pubkey_len as size_t;
    ret = mpi_lset(&mut rsa_ctx.E, 0x10001 as libc::c_int as t_sint);
    if !(ret != 0 as libc::c_int) {
        ret =
            mpi_read_binary(&mut rsa_ctx.P,
                            &mut *(*kd).data.as_mut_ptr().offset(0 as
                                                                     libc::c_int
                                                                     as
                                                                     isize),
                            (pubkey_len / 2 as libc::c_int) as size_t);
        if !(ret != 0 as libc::c_int) {
            ret =
                mpi_read_binary(&mut rsa_ctx.Q,
                                &mut *(*kd).data.as_mut_ptr().offset((pubkey_len
                                                                          /
                                                                          2 as
                                                                              libc::c_int)
                                                                         as
                                                                         isize),
                                (pubkey_len / 2 as libc::c_int) as size_t);
            if !(ret != 0 as libc::c_int) {
                ret =
                    mpi_sub_int(&mut P1, &mut rsa_ctx.P,
                                1 as libc::c_int as t_sint);
                if !(ret != 0 as libc::c_int) {
                    ret =
                        mpi_sub_int(&mut Q1, &mut rsa_ctx.Q,
                                    1 as libc::c_int as t_sint);
                    if !(ret != 0 as libc::c_int) {
                        ret = mpi_mul_mpi(&mut H, &mut P1, &mut Q1);
                        if !(ret != 0 as libc::c_int) {
                            ret =
                                mpi_inv_mod(&mut rsa_ctx.D, &mut rsa_ctx.E,
                                            &mut H);
                            if !(ret != 0 as libc::c_int) {
                                ret =
                                    mpi_mod_mpi(&mut rsa_ctx.DP,
                                                &mut rsa_ctx.D, &mut P1);
                                if !(ret != 0 as libc::c_int) {
                                    ret =
                                        mpi_mod_mpi(&mut rsa_ctx.DQ,
                                                    &mut rsa_ctx.D, &mut Q1);
                                    if !(ret != 0 as libc::c_int) {
                                        ret =
                                            mpi_inv_mod(&mut rsa_ctx.QP,
                                                        &mut rsa_ctx.Q,
                                                        &mut rsa_ctx.P);
                                        (ret) != 0 as libc::c_int;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    mpi_free(&mut P1);
    mpi_free(&mut Q1);
    mpi_free(&mut H);
    if ret == 0 as libc::c_int {
        let mut cs: libc::c_int = 0;
        clp.next = 0 as *mut chx_cleanup;
        clp.routine =
            Some(rsa_cleanup as
                     unsafe extern "C" fn(_: *mut libc::c_void) -> ());
        clp.arg = 0 as *mut libc::c_void;
        chopstx_cleanup_push(&mut clp);
        cs = chopstx_setcancelstate(0 as libc::c_int);
        ret =
            rsa_rsassa_pkcs1_v15_sign(&mut rsa_ctx, None,
                                      0 as *mut libc::c_void,
                                      1 as libc::c_int, 0 as libc::c_int,
                                      msg_len as libc::c_uint, raw_message,
                                      temp.as_mut_ptr());
        memcpy(output as *mut libc::c_void,
               temp.as_mut_ptr() as *const libc::c_void,
               pubkey_len as libc::c_ulong);
        chopstx_setcancelstate(cs);
        chopstx_cleanup_pop(0 as libc::c_int);
    }
    rsa_free(&mut rsa_ctx);
    if ret != 0 as libc::c_int {
        return -(1 as libc::c_int)
    } else {
        set_res_sw(0x90 as libc::c_int as uint8_t,
                   0 as libc::c_int as uint8_t);
        return 0 as libc::c_int
    };
}
/*
 * LEN: length in byte
 */
#[no_mangle]
pub unsafe extern "C" fn modulus_calc(mut p: *const uint8_t,
                                      mut len: libc::c_int) -> *mut uint8_t {
    let mut P: mpi =
        mpi{s: 0,
            n: 0,
            p: 0 as *const t_uint as *mut t_uint,}; /* Allow cancellation.  */
    let mut Q: mpi =
        mpi{s: 0,
            n: 0,
            p: 0 as *const t_uint as *mut t_uint,}; /* Allow cancellation.  */
    let mut N: mpi = mpi{s: 0, n: 0, p: 0 as *const t_uint as *mut t_uint,};
    let mut modulus: *mut uint8_t = 0 as *mut uint8_t;
    let mut ret: libc::c_int = 0;
    modulus = gnuk_malloc(len as size_t) as *mut uint8_t;
    if modulus.is_null() { return 0 as *mut uint8_t }
    mpi_init(&mut P);
    mpi_init(&mut Q);
    mpi_init(&mut N);
    ret = mpi_read_binary(&mut P, p, (len / 2 as libc::c_int) as size_t);
    if !(ret != 0 as libc::c_int) {
        ret =
            mpi_read_binary(&mut Q,
                            p.offset((len / 2 as libc::c_int) as isize),
                            (len / 2 as libc::c_int) as size_t);
        if !(ret != 0 as libc::c_int) {
            ret = mpi_mul_mpi(&mut N, &mut P, &mut Q);
            if !(ret != 0 as libc::c_int) {
                ret = mpi_write_binary(&mut N, modulus, len as size_t);
                (ret) != 0 as libc::c_int;
            }
        }
    }
    mpi_free(&mut P);
    mpi_free(&mut Q);
    mpi_free(&mut N);
    if ret != 0 as libc::c_int {
        return 0 as *mut uint8_t
    } else { return modulus };
}
#[no_mangle]
pub unsafe extern "C" fn rsa_decrypt(mut input: *const uint8_t,
                                     mut output: *mut uint8_t,
                                     mut msg_len: libc::c_int,
                                     mut kd: *mut key_data,
                                     mut output_len_p: *mut libc::c_uint)
 -> libc::c_int {
    let mut P1: mpi = mpi{s: 0, n: 0, p: 0 as *const t_uint as *mut t_uint,};
    let mut Q1: mpi = mpi{s: 0, n: 0, p: 0 as *const t_uint as *mut t_uint,};
    let mut H: mpi = mpi{s: 0, n: 0, p: 0 as *const t_uint as *mut t_uint,};
    let mut ret: libc::c_int = 0;
    rsa_init(&mut rsa_ctx, 0 as libc::c_int, 0 as libc::c_int);
    mpi_init(&mut P1);
    mpi_init(&mut Q1);
    mpi_init(&mut H);
    rsa_ctx.len = msg_len as size_t;
    ret = mpi_lset(&mut rsa_ctx.E, 0x10001 as libc::c_int as t_sint);
    if !(ret != 0 as libc::c_int) {
        ret =
            mpi_read_binary(&mut rsa_ctx.P,
                            &mut *(*kd).data.as_mut_ptr().offset(0 as
                                                                     libc::c_int
                                                                     as
                                                                     isize),
                            (msg_len / 2 as libc::c_int) as size_t);
        if !(ret != 0 as libc::c_int) {
            ret =
                mpi_read_binary(&mut rsa_ctx.Q,
                                &mut *(*kd).data.as_mut_ptr().offset((msg_len
                                                                          /
                                                                          2 as
                                                                              libc::c_int)
                                                                         as
                                                                         isize),
                                (msg_len / 2 as libc::c_int) as size_t);
            if !(ret != 0 as libc::c_int) {
                ret =
                    mpi_sub_int(&mut P1, &mut rsa_ctx.P,
                                1 as libc::c_int as t_sint);
                if !(ret != 0 as libc::c_int) {
                    ret =
                        mpi_sub_int(&mut Q1, &mut rsa_ctx.Q,
                                    1 as libc::c_int as t_sint);
                    if !(ret != 0 as libc::c_int) {
                        ret = mpi_mul_mpi(&mut H, &mut P1, &mut Q1);
                        if !(ret != 0 as libc::c_int) {
                            ret =
                                mpi_inv_mod(&mut rsa_ctx.D, &mut rsa_ctx.E,
                                            &mut H);
                            if !(ret != 0 as libc::c_int) {
                                ret =
                                    mpi_mod_mpi(&mut rsa_ctx.DP,
                                                &mut rsa_ctx.D, &mut P1);
                                if !(ret != 0 as libc::c_int) {
                                    ret =
                                        mpi_mod_mpi(&mut rsa_ctx.DQ,
                                                    &mut rsa_ctx.D, &mut Q1);
                                    if !(ret != 0 as libc::c_int) {
                                        ret =
                                            mpi_inv_mod(&mut rsa_ctx.QP,
                                                        &mut rsa_ctx.Q,
                                                        &mut rsa_ctx.P);
                                        (ret) != 0 as libc::c_int;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    mpi_free(&mut P1);
    mpi_free(&mut Q1);
    mpi_free(&mut H);
    if ret == 0 as libc::c_int {
        let mut cs: libc::c_int = 0;
        clp.next = 0 as *mut chx_cleanup;
        clp.routine =
            Some(rsa_cleanup as
                     unsafe extern "C" fn(_: *mut libc::c_void) -> ());
        clp.arg = 0 as *mut libc::c_void;
        chopstx_cleanup_push(&mut clp);
        cs = chopstx_setcancelstate(0 as libc::c_int);
        ret =
            rsa_rsaes_pkcs1_v15_decrypt(&mut rsa_ctx, None,
                                        0 as *mut libc::c_void,
                                        1 as libc::c_int,
                                        output_len_p as *mut size_t, input,
                                        output,
                                        (5 as libc::c_int + 9 as libc::c_int +
                                             512 as libc::c_int) as size_t);
        chopstx_setcancelstate(cs);
        chopstx_cleanup_pop(0 as libc::c_int);
    }
    rsa_free(&mut rsa_ctx);
    if ret != 0 as libc::c_int {
        return -(1 as libc::c_int)
    } else {
        set_res_sw(0x90 as libc::c_int as uint8_t,
                   0 as libc::c_int as uint8_t);
        return 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn rsa_verify(mut pubkey: *const uint8_t,
                                    mut pubkey_len: libc::c_int,
                                    mut hash: *const uint8_t,
                                    mut sig: *const uint8_t) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    rsa_init(&mut rsa_ctx, 0 as libc::c_int, 0 as libc::c_int);
    rsa_ctx.len = pubkey_len as size_t;
    ret = mpi_lset(&mut rsa_ctx.E, 0x10001 as libc::c_int as t_sint);
    if !(ret != 0 as libc::c_int) {
        ret = mpi_read_binary(&mut rsa_ctx.N, pubkey, pubkey_len as size_t);
        if !(ret != 0 as libc::c_int) {
            ret =
                rsa_rsassa_pkcs1_v15_verify(&mut rsa_ctx, None,
                                            0 as *mut libc::c_void,
                                            0 as libc::c_int,
                                            11 as libc::c_int,
                                            32 as libc::c_int as libc::c_uint,
                                            hash, sig);
            (ret) != 0 as libc::c_int;
        }
    }
    rsa_free(&mut rsa_ctx);
    if ret != 0 as libc::c_int {
        return -(1 as libc::c_int)
    } else { return 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn rsa_genkey(mut pubkey_len: libc::c_int)
 -> *mut uint8_t {
    let mut ret: libc::c_int = 0;
    let mut index: uint8_t = 0 as libc::c_int as uint8_t;
    let mut p_q_modulus: *mut uint8_t =
        gnuk_malloc((pubkey_len * 2 as libc::c_int) as size_t) as
            *mut uint8_t;
    let mut p: *mut uint8_t = p_q_modulus;
    let mut q: *mut uint8_t =
        p_q_modulus.offset((pubkey_len / 2 as libc::c_int) as isize);
    let mut modulus: *mut uint8_t = p_q_modulus.offset(pubkey_len as isize);
    let mut cs: libc::c_int = 0;
    extern "C" {
        #[link_name = "prng_seed"]
        fn prng_seed_0(f_rng:
                           Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                       _: *mut libc::c_uchar,
                                                       _: size_t)
                                      -> libc::c_int>,
                       p_rng: *mut libc::c_void) -> libc::c_int;
    }
    extern "C" {
        #[link_name = "neug_flush"]
        fn neug_flush_0();
    }
    if p_q_modulus.is_null() { return 0 as *mut uint8_t }
    neug_flush_0();
    prng_seed_0(Some(random_gen as
                         unsafe extern "C" fn(_: *mut libc::c_void,
                                              _: *mut libc::c_uchar,
                                              _: size_t) -> libc::c_int),
                &mut index as *mut uint8_t as *mut libc::c_void);
    rsa_init(&mut rsa_ctx, 0 as libc::c_int, 0 as libc::c_int);
    clp.next = 0 as *mut chx_cleanup;
    clp.routine =
        Some(rsa_cleanup as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    clp.arg = p_q_modulus as *mut libc::c_void;
    chopstx_cleanup_push(&mut clp);
    cs = chopstx_setcancelstate(0 as libc::c_int);
    ret =
        rsa_gen_key(&mut rsa_ctx,
                    Some(random_gen as
                             unsafe extern "C" fn(_: *mut libc::c_void,
                                                  _: *mut libc::c_uchar,
                                                  _: size_t) -> libc::c_int),
                    &mut index as *mut uint8_t as *mut libc::c_void,
                    (pubkey_len * 8 as libc::c_int) as libc::c_uint,
                    0x10001 as libc::c_int);
    if !(ret != 0 as libc::c_int) {
        if ret != 0 as libc::c_int {
            chopstx_setcancelstate(cs);
            chopstx_cleanup_pop(0 as libc::c_int);
            gnuk_free(p_q_modulus as *mut libc::c_void);
            rsa_free(&mut rsa_ctx);
            return 0 as *mut uint8_t
        }
        ret =
            mpi_write_binary(&mut rsa_ctx.P, p,
                             (pubkey_len / 2 as libc::c_int) as size_t);
        if !(ret != 0 as libc::c_int) {
            ret =
                mpi_write_binary(&mut rsa_ctx.Q, q,
                                 (pubkey_len / 2 as libc::c_int) as size_t);
            if !(ret != 0 as libc::c_int) {
                ret =
                    mpi_write_binary(&mut rsa_ctx.N, modulus,
                                     pubkey_len as size_t);
                (ret) != 0 as libc::c_int;
            }
        }
    }
    chopstx_setcancelstate(cs);
    chopstx_cleanup_pop(0 as libc::c_int);
    rsa_free(&mut rsa_ctx);
    if ret != 0 as libc::c_int {
        return 0 as *mut uint8_t
    } else { return p_q_modulus };
}
