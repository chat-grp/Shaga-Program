/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as web3 from '@solana/web3.js'
import * as beet from '@metaplex-foundation/beet'
import * as beetSolana from '@metaplex-foundation/beet-solana'

/**
 * Arguments used to create {@link Rental}
 * @category Accounts
 * @category generated
 */
export type RentalArgs = {
  client: web3.PublicKey
  affair: web3.PublicKey
  rentAmount: beet.bignum
  rentalStartTime: beet.bignum
  rentalTerminationTime: beet.bignum
  rentalClockworkThreadId: web3.PublicKey
}

export const rentalDiscriminator = [121, 83, 229, 235, 73, 50, 143, 184]
/**
 * Holds the data for the {@link Rental} Account and provides de/serialization
 * functionality for that data
 *
 * @category Accounts
 * @category generated
 */
export class Rental implements RentalArgs {
  private constructor(
    readonly client: web3.PublicKey,
    readonly affair: web3.PublicKey,
    readonly rentAmount: beet.bignum,
    readonly rentalStartTime: beet.bignum,
    readonly rentalTerminationTime: beet.bignum,
    readonly rentalClockworkThreadId: web3.PublicKey
  ) {}

  /**
   * Creates a {@link Rental} instance from the provided args.
   */
  static fromArgs(args: RentalArgs) {
    return new Rental(
      args.client,
      args.affair,
      args.rentAmount,
      args.rentalStartTime,
      args.rentalTerminationTime,
      args.rentalClockworkThreadId
    )
  }

  /**
   * Deserializes the {@link Rental} from the data of the provided {@link web3.AccountInfo}.
   * @returns a tuple of the account data and the offset up to which the buffer was read to obtain it.
   */
  static fromAccountInfo(
    accountInfo: web3.AccountInfo<Buffer>,
    offset = 0
  ): [Rental, number] {
    return Rental.deserialize(accountInfo.data, offset)
  }

  /**
   * Retrieves the account info from the provided address and deserializes
   * the {@link Rental} from its data.
   *
   * @throws Error if no account info is found at the address or if deserialization fails
   */
  static async fromAccountAddress(
    connection: web3.Connection,
    address: web3.PublicKey,
    commitmentOrConfig?: web3.Commitment | web3.GetAccountInfoConfig
  ): Promise<Rental> {
    const accountInfo = await connection.getAccountInfo(
      address,
      commitmentOrConfig
    )
    if (accountInfo == null) {
      throw new Error(`Unable to find Rental account at ${address}`)
    }
    return Rental.fromAccountInfo(accountInfo, 0)[0]
  }

  /**
   * Provides a {@link web3.Connection.getProgramAccounts} config builder,
   * to fetch accounts matching filters that can be specified via that builder.
   *
   * @param programId - the program that owns the accounts we are filtering
   */
  static gpaBuilder(
    programId: web3.PublicKey = new web3.PublicKey(
      '9SwYZxTQUYruFSHYeTqrtB5pTtuGJEGksh7ufpNS1YK5'
    )
  ) {
    return beetSolana.GpaBuilder.fromStruct(programId, rentalBeet)
  }

  /**
   * Deserializes the {@link Rental} from the provided data Buffer.
   * @returns a tuple of the account data and the offset up to which the buffer was read to obtain it.
   */
  static deserialize(buf: Buffer, offset = 0): [Rental, number] {
    return rentalBeet.deserialize(buf, offset)
  }

  /**
   * Serializes the {@link Rental} into a Buffer.
   * @returns a tuple of the created Buffer and the offset up to which the buffer was written to store it.
   */
  serialize(): [Buffer, number] {
    return rentalBeet.serialize({
      accountDiscriminator: rentalDiscriminator,
      ...this,
    })
  }

  /**
   * Returns the byteSize of a {@link Buffer} holding the serialized data of
   * {@link Rental}
   */
  static get byteSize() {
    return rentalBeet.byteSize
  }

  /**
   * Fetches the minimum balance needed to exempt an account holding
   * {@link Rental} data from rent
   *
   * @param connection used to retrieve the rent exemption information
   */
  static async getMinimumBalanceForRentExemption(
    connection: web3.Connection,
    commitment?: web3.Commitment
  ): Promise<number> {
    return connection.getMinimumBalanceForRentExemption(
      Rental.byteSize,
      commitment
    )
  }

  /**
   * Determines if the provided {@link Buffer} has the correct byte size to
   * hold {@link Rental} data.
   */
  static hasCorrectByteSize(buf: Buffer, offset = 0) {
    return buf.byteLength - offset === Rental.byteSize
  }

  /**
   * Returns a readable version of {@link Rental} properties
   * and can be used to convert to JSON and/or logging
   */
  pretty() {
    return {
      client: this.client.toBase58(),
      affair: this.affair.toBase58(),
      rentAmount: (() => {
        const x = <{ toNumber: () => number }>this.rentAmount
        if (typeof x.toNumber === 'function') {
          try {
            return x.toNumber()
          } catch (_) {
            return x
          }
        }
        return x
      })(),
      rentalStartTime: (() => {
        const x = <{ toNumber: () => number }>this.rentalStartTime
        if (typeof x.toNumber === 'function') {
          try {
            return x.toNumber()
          } catch (_) {
            return x
          }
        }
        return x
      })(),
      rentalTerminationTime: (() => {
        const x = <{ toNumber: () => number }>this.rentalTerminationTime
        if (typeof x.toNumber === 'function') {
          try {
            return x.toNumber()
          } catch (_) {
            return x
          }
        }
        return x
      })(),
      rentalClockworkThreadId: this.rentalClockworkThreadId.toBase58(),
    }
  }
}

/**
 * @category Accounts
 * @category generated
 */
export const rentalBeet = new beet.BeetStruct<
  Rental,
  RentalArgs & {
    accountDiscriminator: number[] /* size: 8 */
  }
>(
  [
    ['accountDiscriminator', beet.uniformFixedSizeArray(beet.u8, 8)],
    ['client', beetSolana.publicKey],
    ['affair', beetSolana.publicKey],
    ['rentAmount', beet.u64],
    ['rentalStartTime', beet.u64],
    ['rentalTerminationTime', beet.u64],
    ['rentalClockworkThreadId', beetSolana.publicKey],
  ],
  Rental.fromArgs,
  'Rental'
)