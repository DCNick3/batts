const express = require('express')
const cors = require('cors')

const router = express.Router()

const PORT = 3000

//@ts-ignore
const succeed = (data, status=200) => (_req, res) => {
    res.status(status).send({
        status: 'Success',
        payload: data
    })
}
//@ts-ignore
const fail = (data, status=404) => (_req, res) => {
    res.status(status).send({
        status: 'Error',
        payload: data
    })
}

router.get('/users/me', succeed(require('./data/me.json')))
// router.get('/users/me', fail(require('./data/me.404.json')))

router.get('/users/:id/profile', succeed(require('./data/user.json')))
// router.get('/users/:id', fail(require('./data/?')))

router.get('/tickets/owned', succeed(require('./data/ownedTickets.json')))
// router.get('/tickets/owned', fail(require('./data/?')))

router.get('/tickets/assigned', succeed(require('./data/assignedTickets.json')))
// router.get('/tickets/assigned', fail(require('./data/?')))

router.get('/tickets/:id', succeed(require('./data/ticket.json')))
// router.get('/tickets/:id', fail(require('./data/ticket.404.json')))
router.post('/tickets/:id', succeed(require('./data/ticket.json')))

router.get('/groups/:id', succeed(require('./data/group.json')))
// router.get('/groups/:id', fail(require(./data/?)))
router.post('/groups/:id', succeed(require('./data/group.json')))

router.get('/users/:id/groups/', succeed(require('./data/groups.json')))
// router.get('/groups/:id', fail(require('./data/?')))

const app = express()
app.use(cors())
app.use('/api', router)
console.log("Started")
app.listen(PORT)
