export type Update
  = { type: 'AddUser', id: string }
  | { type: 'DeleteUser', id: string }

export class Updates {
  deletes: Set<string> = new Set()
  adds: Set<string> = new Set()

  constructor() {
  }

  add(uid: string) {
    this.deletes.delete(uid)
    this.adds.add(uid)
  }

  delete(uid: string) {
    this.deletes.add(uid)
    this.adds.delete(uid)
  }

  getUpdates() {
    return Array.from(this.adds).map(id => ({ type: 'AddUser', id })).concat(
      Array.from(this.deletes).map(id => ({ type: 'DeleteUser', id }))
    )
  }

  updatesPresent() {
    return this.adds.size > 0 || this.deletes.size > 0
  }
}
