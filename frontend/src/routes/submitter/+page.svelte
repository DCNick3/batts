<script>
	export let data;
</script>

<div class="container">
	<h1>Form</h1>

	<form
		method="POST"
		action="?/create"
	>
		
		<label>
			Submit To:
			<select>
				{#each data.receivers as receiver}
					<option value={receiver.id}>{receiver.name}</option>
				{/each}
			</select>
		</label>
		<label>
			Topic:
			<input
				name="topic"
				required
			/>
		</label>
		<label>
			Description:
			<textarea
				name="description"
				required
			/>
		</label>
	</form>

	<button>Submit</button>

	<h1>Submitted requests</h1>

	<ul>
		{#each data.requests as request}
			<li><strong>To:</strong> {request.receiver}, <strong>Topic:</strong> {request.topic}, <strong>Status:</strong> {request.status}</li>
		{/each}
	</ul>
</div>


<style>
	.container {
		max-width: 25em;
		margin: 0 auto;
		display: flex;
		flex-direction: column;
		gap: 20px;
	}

	h1 {
		margin: 40px auto 0;
	}

	form {
		display: flex;
		flex-direction: column;
		gap: 15px;
	}
	label {
		width: 100%;
		display: flex;
		flex-direction: column;
		gap: 3px;
	}

	select, input, textarea {
		flex: 1;
		padding: 0.5rem;
		border: 1px solid hsl(206, 20%, 90%);
		border-radius: 4px;
		box-sizing: border-box;
		background-color: white;
	}

	/* textarea {
		font-family: inherit;
	} */

	ul {
		margin: 0;
		padding: 0;
		list-style: none;
	}

	li {
		margin: 10px 0;
	}

	button {
		cursor: pointer;
		height: 30px;
		width: 55%;
		margin: 0 auto;
		aspect-ratio: 1;
		opacity: 0.5;
		transition: opacity 0.2s;
	}

	button:hover {
		opacity: 1;
	}
</style>