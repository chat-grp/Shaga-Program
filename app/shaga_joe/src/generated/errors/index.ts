/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

type ErrorWithCode = Error & { code: number }
type MaybeErrorWithCode = ErrorWithCode | null | undefined

const createErrorFromCodeLookup: Map<number, () => ErrorWithCode> = new Map()
const createErrorFromNameLookup: Map<string, () => ErrorWithCode> = new Map()

/**
 * InvalidAffair: 'Invalid Session'
 *
 * @category Errors
 * @category generated
 */
export class InvalidAffairError extends Error {
  readonly code: number = 0x1770
  readonly name: string = 'InvalidAffair'
  constructor() {
    super('Invalid Session')
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, InvalidAffairError)
    }
  }
}

createErrorFromCodeLookup.set(0x1770, () => new InvalidAffairError())
createErrorFromNameLookup.set('InvalidAffair', () => new InvalidAffairError())

/**
 * InvalidLender: 'Invalid Lender'
 *
 * @category Errors
 * @category generated
 */
export class InvalidLenderError extends Error {
  readonly code: number = 0x1771
  readonly name: string = 'InvalidLender'
  constructor() {
    super('Invalid Lender')
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, InvalidLenderError)
    }
  }
}

createErrorFromCodeLookup.set(0x1771, () => new InvalidLenderError())
createErrorFromNameLookup.set('InvalidLender', () => new InvalidLenderError())

/**
 * InvalidPayload: 'Invalid Payload'
 *
 * @category Errors
 * @category generated
 */
export class InvalidPayloadError extends Error {
  readonly code: number = 0x1772
  readonly name: string = 'InvalidPayload'
  constructor() {
    super('Invalid Payload')
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, InvalidPayloadError)
    }
  }
}

createErrorFromCodeLookup.set(0x1772, () => new InvalidPayloadError())
createErrorFromNameLookup.set('InvalidPayload', () => new InvalidPayloadError())

/**
 * AffairListFull: 'Sessions List Full'
 *
 * @category Errors
 * @category generated
 */
export class AffairListFullError extends Error {
  readonly code: number = 0x1773
  readonly name: string = 'AffairListFull'
  constructor() {
    super('Sessions List Full')
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, AffairListFullError)
    }
  }
}

createErrorFromCodeLookup.set(0x1773, () => new AffairListFullError())
createErrorFromNameLookup.set('AffairListFull', () => new AffairListFullError())

/**
 * ClientAlreadyInAffair: 'Client Already in a Session'
 *
 * @category Errors
 * @category generated
 */
export class ClientAlreadyInAffairError extends Error {
  readonly code: number = 0x1774
  readonly name: string = 'ClientAlreadyInAffair'
  constructor() {
    super('Client Already in a Session')
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, ClientAlreadyInAffairError)
    }
  }
}

createErrorFromCodeLookup.set(0x1774, () => new ClientAlreadyInAffairError())
createErrorFromNameLookup.set(
  'ClientAlreadyInAffair',
  () => new ClientAlreadyInAffairError()
)

/**
 * InsufficientFunds: 'Insufficient Funds'
 *
 * @category Errors
 * @category generated
 */
export class InsufficientFundsError extends Error {
  readonly code: number = 0x1775
  readonly name: string = 'InsufficientFunds'
  constructor() {
    super('Insufficient Funds')
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, InsufficientFundsError)
    }
  }
}

createErrorFromCodeLookup.set(0x1775, () => new InsufficientFundsError())
createErrorFromNameLookup.set(
  'InsufficientFunds',
  () => new InsufficientFundsError()
)

/**
 * InvalidRentalTerminationTime: 'Invalid Rental Termination Time'
 *
 * @category Errors
 * @category generated
 */
export class InvalidRentalTerminationTimeError extends Error {
  readonly code: number = 0x1776
  readonly name: string = 'InvalidRentalTerminationTime'
  constructor() {
    super('Invalid Rental Termination Time')
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, InvalidRentalTerminationTimeError)
    }
  }
}

createErrorFromCodeLookup.set(
  0x1776,
  () => new InvalidRentalTerminationTimeError()
)
createErrorFromNameLookup.set(
  'InvalidRentalTerminationTime',
  () => new InvalidRentalTerminationTimeError()
)

/**
 * InvalidTerminationTime: 'Invalid Termination Time'
 *
 * @category Errors
 * @category generated
 */
export class InvalidTerminationTimeError extends Error {
  readonly code: number = 0x1777
  readonly name: string = 'InvalidTerminationTime'
  constructor() {
    super('Invalid Termination Time')
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, InvalidTerminationTimeError)
    }
  }
}

createErrorFromCodeLookup.set(0x1777, () => new InvalidTerminationTimeError())
createErrorFromNameLookup.set(
  'InvalidTerminationTime',
  () => new InvalidTerminationTimeError()
)

/**
 * AffairAlreadyJoined: 'Session Occupied'
 *
 * @category Errors
 * @category generated
 */
export class AffairAlreadyJoinedError extends Error {
  readonly code: number = 0x1778
  readonly name: string = 'AffairAlreadyJoined'
  constructor() {
    super('Session Occupied')
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, AffairAlreadyJoinedError)
    }
  }
}

createErrorFromCodeLookup.set(0x1778, () => new AffairAlreadyJoinedError())
createErrorFromNameLookup.set(
  'AffairAlreadyJoined',
  () => new AffairAlreadyJoinedError()
)

/**
 * ThreadInitializationFailed: 'Thread Initialization Failed'
 *
 * @category Errors
 * @category generated
 */
export class ThreadInitializationFailedError extends Error {
  readonly code: number = 0x1779
  readonly name: string = 'ThreadInitializationFailed'
  constructor() {
    super('Thread Initialization Failed')
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, ThreadInitializationFailedError)
    }
  }
}

createErrorFromCodeLookup.set(
  0x1779,
  () => new ThreadInitializationFailedError()
)
createErrorFromNameLookup.set(
  'ThreadInitializationFailed',
  () => new ThreadInitializationFailedError()
)

/**
 * MissingRentalContext: 'Missing Rental Context for Session Termination'
 *
 * @category Errors
 * @category generated
 */
export class MissingRentalContextError extends Error {
  readonly code: number = 0x177a
  readonly name: string = 'MissingRentalContext'
  constructor() {
    super('Missing Rental Context for Session Termination')
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, MissingRentalContextError)
    }
  }
}

createErrorFromCodeLookup.set(0x177a, () => new MissingRentalContextError())
createErrorFromNameLookup.set(
  'MissingRentalContext',
  () => new MissingRentalContextError()
)

/**
 * InvalidRentalContext: 'Wrong Rental Context for Session Termination'
 *
 * @category Errors
 * @category generated
 */
export class InvalidRentalContextError extends Error {
  readonly code: number = 0x177b
  readonly name: string = 'InvalidRentalContext'
  constructor() {
    super('Wrong Rental Context for Session Termination')
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, InvalidRentalContextError)
    }
  }
}

createErrorFromCodeLookup.set(0x177b, () => new InvalidRentalContextError())
createErrorFromNameLookup.set(
  'InvalidRentalContext',
  () => new InvalidRentalContextError()
)

/**
 * UnauthorizedAffairCreation: 'Only registered lenders can create affairs'
 *
 * @category Errors
 * @category generated
 */
export class UnauthorizedAffairCreationError extends Error {
  readonly code: number = 0x177c
  readonly name: string = 'UnauthorizedAffairCreation'
  constructor() {
    super('Only registered lenders can create affairs')
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, UnauthorizedAffairCreationError)
    }
  }
}

createErrorFromCodeLookup.set(
  0x177c,
  () => new UnauthorizedAffairCreationError()
)
createErrorFromNameLookup.set(
  'UnauthorizedAffairCreation',
  () => new UnauthorizedAffairCreationError()
)

/**
 * Attempts to resolve a custom program error from the provided error code.
 * @category Errors
 * @category generated
 */
export function errorFromCode(code: number): MaybeErrorWithCode {
  const createError = createErrorFromCodeLookup.get(code)
  return createError != null ? createError() : null
}

/**
 * Attempts to resolve a custom program error from the provided error name, i.e. 'Unauthorized'.
 * @category Errors
 * @category generated
 */
export function errorFromName(name: string): MaybeErrorWithCode {
  const createError = createErrorFromNameLookup.get(name)
  return createError != null ? createError() : null
}